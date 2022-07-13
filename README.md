# SKRS
I do wonder if renderer is bugged, like some carry over between frames
works for garbage, needs to do like 'command not found'
list doesnt clear?


probably need to sort transparent geometry. it is sorted. why is it so fucked up?
terminal needs outputs etc

dont close without savings

maybe it would be good if levels had a UUID and coming out of terminal it got checked and UUIDs were always valid
or I could just terminal_print from main that it not valid
yea actually terminal no can access

terminal no clear sometimes

I think cmds should be
 * new X
 * load X
 * save
 * play [autosaves]
and probably indicate there are unsaved changes like in the window title

literally going to spin on level structure forever, maybe just a menu
maybe make the portals better though, like indicate what it is. You can express different things like elaboration.

way back to editor from ingame?

god damn move that terminal code somewhere
can it return an enum?

pow crate
tree pow

hmm achievements or gates
like it doesnt make any sense, achievements could be a pickup
funny if the puzzle elements annihilated
box wrapper

definitely have portals gated by # achievements

path block



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


bug: possession mode: would actually be a great puzzle mechanic. just have a bunch of things interacting
kinda like baba is you but make it about jumping from thing to thing. maybe if you try and move into something you geist into it. cant go from somethings to other things


bug: level titles need to line up


it would probably be more immersive if there was a 1:1 coro with going in/out of portals hey.
or even no portals, going off edges in certain spots and just gates of certain requirement

maybe there should be 'return portals': always free, always back wehre u came from

some kind of ctx stack i guess

it probably would look sick if tiles dissolved in and out
portals should be super minimal, maybe only active if youre near as well
green = level has been done

certainly got some polishing to do