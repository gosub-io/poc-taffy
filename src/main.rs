use taffy::prelude::*;
use crossterm::{
    event::{self, KeyCode,},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{prelude::{CrosstermBackend, Terminal}, widgets::{Block}, layout::{Rect}, Frame};
use std::io::{stdout, Result};
use ratatui::prelude::Color;
use ratatui::prelude::Style as uiStyle;
use ratatui::widgets::Borders;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


fn main() -> Result<()> {
    let (mut tree, root_node) = generate_taffy_tree();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut last_computed_size = None;

    terminal.clear()?;
    loop {
        terminal.draw(|frame| {
            // Recalculate layout if the frame has changed sizing
            if last_computed_size != Some(frame.size()) {
                let viewport_size = Size {
                    width: AvailableSpace::Definite(frame.size().width as f32),
                    height: AvailableSpace::Definite(frame.size().height as f32),
                };
                tree.compute_layout(root_node, viewport_size).unwrap();

                last_computed_size = Some(frame.size());
            }

            // Set the PRNG seed so each render uses the same (random) color for each block
            let mut prng = StdRng::seed_from_u64(0);
            create_layout(&mut prng, &mut tree, root_node, frame);
        })?;

        if event::poll(std::time::Duration::from_millis(15))? {
            if let event::Event::Key(event) = event::read()? {
                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn create_layout(prng: &mut StdRng, tree: &TaffyTree, node_id: NodeId, frame: &mut Frame) {
    let layout = tree.get_final_layout(node_id);

    let r = Rect::new(
        layout.location.x as u16,
        layout.location.y as u16,
        layout.size.width as u16,
        layout.size.height as u16
    );

    let bg = Color::Rgb(prng.gen_range(0..=255), prng.gen_range(0..=255), prng.gen_range(0..=255));
    let b = Block::new()
        .title(format!("{:?}", node_id))
        .borders(Borders::ALL)
        .style(uiStyle::default().bg(bg).fg(Color::White))
    ;

    frame.render_widget(b, r);

    for child_node_id in tree.child_ids(node_id) {
        create_layout(prng, tree, child_node_id, frame);
    }
}

fn generate_taffy_tree() -> (TaffyTree, NodeId) {
    let mut tree:  TaffyTree<()> = TaffyTree::new();

    /// Children for the third node
    let c3_1 = tree.new_leaf(Style{
        size: Size { width: Dimension::Percent(0.5), height: Dimension::Length(10.0)},
        ..Default::default()
    }).unwrap();

    let c3_2 = tree.new_leaf(Style{
        size: Size { width: Dimension::Percent(0.75), height: Dimension::Length(20.0)},
        align_content: Option::from(AlignContent::FlexEnd),
        ..Default::default()
    }).unwrap();

    /// The main three nodes of the tree directly under the root node
    let c1 = tree.new_leaf(Style{
        size: Size { width: Dimension::Percent(0.5), height: Dimension::Length(10.0)},
        ..Default::default()
    }).unwrap();
    let c2 = tree.new_leaf(Style{
        size: Size { width: Dimension::Percent(0.25), height: Dimension::Percent(0.25)},
        ..Default::default()
    }).unwrap();
    let c3 = tree.new_with_children(Style{
        size: Size { width: Dimension::Percent(0.9), height: Dimension::Length(30.0)},
        ..Default::default()
    }, &[c3_1, c3_2]).unwrap();

    // Root node
    let root_node = tree.new_with_children(
        Style{
            size: Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0)},
            display: Display::Block,
            // flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        &[c1, c2, c3]
    ).unwrap();


/*
    The tree looks like this:

        root
       / | \
      c1 c2 c3
           /  \
         c3_1 c3_2

 */

    (tree, root_node)
}
