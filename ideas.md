# Zetarune

Game engine for faithfully making deltarune-like stuff. Game save compatibility is NOT a goal, so you will NOT be able to import saves from vanilla or a mod. NO included assets or code.

## Requirements

- Able to import deltarune assets easily (likely use libgm for this)
- POSSIBLY able to import GML in some limited cases (Underanalyzer/UTMT CLI + custom converter from GML to Rust, will only work rarely)
- Able to import deltarune dialogue and convert it to my custom format
- Able to import deltarune rooms (at least limited)
- Able to easily make and edit rooms (custom GUI)
- Proper UI
- Both light world and dark world
- Ergonomic API (likely using proc macros to make it feel better and so it can import stuff from files (i.e. generated files by the room editor))
- Both GUI and direct file editing supported (use vscode for GUI to make it more integrated?)
- Proper battles with custom bullet shapes and bullet patterns (likely use some code to generate the patterns using a variety of variables from the game state)
    - Custom actions and everything on a per-enemy basis
- Party changes throughout the game
- Animations/cutscenes
- Scripting (Rust)/triggers
    - Touch/proximity triggers
    - Interact triggers
- Saving/loading
- Production of an installer which can create the actual game by importing either a data.win file or for games which use assets from multiple chapters the entire deltarune folder (only needed if the user actually uses assets from the vanilla game)
    - Likely works by just having the GUI produce the code which is compiled by rustc/cargo to the target platform and is then the installer or whatever and the GUI automatically chooses whether it is an installer for one chapter, multiple, or just the direct game and from that sets a cargo feature on the framework which changes how the produced executable works
- Extensible menu system
- Audio system with various options for crossfades, cuts, channels, positional, dynamic transitions, etc
- Layered sprites/objects
- Object groups (an object that just consists of a bunch of sub-objects, layered objects might be this)
- Debugging+incremental compilation
- Dialogue system/translatable text system
    - Potentially a custom patcher which takes an installer executable/data file from zetarune and adds other translations to it
- Adding new assets
- Documentation and templates
- Character and inventory systems (different stats for characters changable throughout, inventories for characters, for the party, and storage)
- Collision (duh)
- Game state (tied into saving, saving most likely will save everything in some object that is passed around to all scripts, containers and some other objects automatically save their state unless overridden)
- Enemies in the overworld (various sprites, animations, states, etc)
- Enemy stats
- Encounters and encounter API to trigger encounters from a script
- Visual effects (API TBD)
- State machine determining what is activated in the game and whether the player is in a battle, dark world, light world, menu, save menu, etc
- Soul modes
- Game over
- Multiple input methods (input and video library TBD, minifb is good but doesn't have gamepad support)
- Error handling (honestly probably just redefine the panic macro as one that triggers a custom error handler in user scripts, `#![forbid(clippy::panic)]` (to prevent reimporting), and always run clippy before compiling, while also exposing the custom error handling macro as a proper name for people who don't want to get confused. and also use catch_unwind in case people are sneaky)
- Modding (ideally allow importing a data file produced by a game and using it like a deltarune data.win file, but that's later)

## Ideas

GUI/interface will likely be something which produces code for the library/framework so people could write stuff directly with the code but the GUI makes it easier.
