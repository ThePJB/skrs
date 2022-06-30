# SKRS

OK but yea anyway we've got play
need to return from play to wherever we were
save, load

lool "notice of achievement"

we could just go linked list style with next pointers as portals
or we could go hub style, hub better if people get stuck
do we want always to have portal back if its hub style?

could have trees we can walk under but not push block under!
and able to be pow'd, leaving stump

definitely have portals gated by # achievements
and i think linked list mode as desired form of expression e.g. 'im teaching you this'
cool portal rendering will be good

tbh fuck right menu, have terminal down the bottom
   * play
   * help
   * list
   * save <name>
   * load <name>
   * link <name>
   * dims <w> <h>

maybe session separate editor function
and separate terminal function probably

level repository save to disk




OK ive figured out structure of levels, worlds. have it be in engine constantly, global level registry, levels referenced by name
drop into edit mode, load, save and test levels.


maybe a subtle path block can be used for expression / hints? the path can lie lol. some environment might be appreciated


* todo animation, more mechanics etc.
* classic problem of how am i gonna structure levels, worlds. i reckon level packets you can go next, prev in. allow 1 skip or something

lol it would be funny if the tiles all flew offscreen and reassembled the level
tripper puzzle game. i reckon a cooler aesthetic is possible. trippy would be good for effects

Pow box tho!!
explode on push into wall? what about on slide into wall
for deleting boxes, as a hazard

maybe teach them the thing in a really obvious way, then make them find it when its not obvious: force to generalize

recognition of achievement token

## Bugs
* spurious gaps in renderer are yuck


I should probably have an object that just stores like owned information about spritesheet etc? eh but yuck

level ideas
 * want to keep it reasonable per world, no ugly ones, really hold peoples hand through the concepts hey, and have levels relate to one another

theorems
 * stuck to wall - a classic
 * order matters - a classic
 * use present as box first
 * block for slide off on ice
 * orthogonal to not slide
 * stuck in ice - no going back - should do a basic one of that - do at once
 * all at once send off
 * the corner one I dont like, it at least reads, how do I go back
    * could use present to prepare the thing to go back