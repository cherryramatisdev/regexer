use anathema::core::views::View;
use anathema::core::{Event, KeyCode, Nodes};
use anathema::values::{State, StateValue};
use anathema::{runtime::Runtime, vm::Templates};

use regexer::parse;

#[derive(Debug, State)]
struct RootState {
    input: StateValue<String>,
    cursor_pos: StateValue<usize>,
    output: StateValue<String>,
}

struct RootView {
    state: RootState,
}

impl View for RootView {
    type State = RootState;

    fn on_event(&mut self, event: Event, _nodes: &mut Nodes<'_>) {
        if let Event::KeyPress(code, ..) = event {
            match code {
                KeyCode::Left if *self.state.cursor_pos > 0 => *self.state.cursor_pos -= 1,
                KeyCode::Right => {
                    if self.state.input.is_empty() {
                        return;
                    }

                    if *self.state.cursor_pos <= self.state.input.chars().count() - 1 {
                        *self.state.cursor_pos += 1;
                    }
                }
                KeyCode::Char(c) => {
                    if *self.state.cursor_pos == self.state.input.chars().count() {
                        *self.state.cursor_pos += 1;
                        self.state.input.push(c);
                    } else {
                        self.state.input.insert(*self.state.cursor_pos, c);
                        *self.state.cursor_pos += 1;
                    }
                }
                KeyCode::Backspace => {
                    if *self.state.cursor_pos == 0 {
                        return;
                    }

                    if *self.state.cursor_pos == self.state.input.chars().count() {
                        *self.state.cursor_pos -= 1;
                        self.state.input.pop();
                    } else {
                        self.state.input.remove(*self.state.cursor_pos);
                        *self.state.cursor_pos -= 1;
                    }
                }
                KeyCode::Enter => {
                    let regex = parse(self.state.input.to_string());
                    *self.state.output = regex;
                }
                _ => {}
            }
        }
    }

    fn state(&self) -> &dyn State {
        &self.state
    }
}

pub fn call() -> anyhow::Result<()> {
    let root_view = RootView {
        state: RootState {
            input: String::new().into(),
            output: String::new().into(),
            cursor_pos: 0.into(),
        },
    };

    let template = include_str!("template.tiny").to_string();

    let mut templates = Templates::new(template, root_view);

    templates.compile()?;

    let mut runtime = Runtime::new(templates.expressions())?;

    runtime.enable_ctrlc = true;
    runtime.enable_tabindex = false;

    // NOTE: remove this comment to debug
    // runtime.enable_alt_screen = false;

    runtime.run()?;

    Ok(())
}
