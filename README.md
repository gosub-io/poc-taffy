# Taffy proof of concept

This is a simple proof of concept for taffy in combination with ratatui.

First, we generate a taffy tree (`generate_taffy_tree`). Each tree leaf has its own settings like size, alignment etc. 
These can all be generated from CSS information in a later stage.

Once we have the taffy tree, we can enter the main ratatui loop. This will do two things:

 - compute the actual layout of the taffy tree, based on the dimensions given.
 - render the taffy tree to the terminal.

It's possible that the screen changes size, in which case we need to recompute the layout. This will only be done
on each change in size, not on each frame.

The taffy tree is rendered to the terminal using the `ratatui` library. It calls `create_layout` with a taffy node
at which point it can generate a simple block. This block is then rendered to the terminal.
Each of the leaves of the tree is rendered to the terminal in the same way.