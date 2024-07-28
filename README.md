# MMSpellbook
MMSpellbook (Magic Modelling Spellbook) is a magic system for Godot that allows for the creation of custom spells. Once complete, it should provide a way for players to write their own custom spells using a simple language in game and then be executed. The spell has an amount of energy and that energy is used as the spell executes. Different instructions in the spell have different efficiencies which is a factor in how much energy that instruction uses. The spell executes until it runs out of energy.

## How to use
- Download the repository. You can the command `git clone https://github.com/CocytusDEDI/MMSpellbook.git` in the terminal if you have git installed
- Put `MMSpellbook.gdextension` into your godot project and change the paths to where the compiled library would be
- Compile the rust code using `cargo build` while in the repository folder (if you don't have rust installed, install it from the rust website)
- Use the code in `player.gd` example in the examples folder. This code should be used alongside your already existing player code

## Optional setup
- Forms: This feature is designed to give the game developer more freedom in spell variety and add give the spell custom visuals. Forms give the player the ability to call the components `take_form` and `undo_form` which allow a single scene to be added as a child of the spell. These scenes are specified by the game developer in the `config.toml` file under the `[forms]` section. Simply equate a the number you want the form to have a dictionary with keys `path` and `energy_requirement` where `path` is the path to a packed scene along with it's name and file extension and `energy_requirement` is a number which decides the base energy required to cast the component (base energy is the energy needed to cast a component when at 100% efficiency). You can checkout `examples/config.toml` for an example.

## Missing features / Issues / Planning to change
- Lacking enough components
- Custom materials and meshes can't be loaded for spells
- Can't specify colour per spell

## Debugging
Start Godot via the terminal so you can see detailed error messages.
