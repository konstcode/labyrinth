# Labyrinth

Main description [here](https://github.com/CleanCut/rusty_engine/blob/main/scenarios/labrinth.md).

## Concept

* In one screen we can see rounded or square labyrinth(possible few levels/maps). There are also have start zone 
  where the ball/marble located. When we tilting field with mouse or possible with arrow keys or wasd, ball 
  start roll to the direction of tilting.
* Labyrinth consist of walls that marble collide with and can't pass through.
* Speed and direction depends on angle and angle of tilting of map/field.
* Possible task is auto-generation of labyrinth with some graph theory.

## Objects

* Board - a field where all happen that can be tilted and have animation of shadowing or 3D like perspective.
* Wall - prevent marble to pass through on collision and have same 3D effect as board.
* Marble - have speed, acceleration, direction of roll depends on board tilting.
* StartPoint - where marble appears at the beginning.
* FinishPoint - win if marble got there.
* Hole - if marble collide with it - game over.

## Investigation #todo

- [x] **rusty engine** - simplified game engine, wrapper on real **bevy**. Configure basic setup and make something simple.
- [ ] move marble while tilting board: speed, acceleration/deceleration, direction.
- [ ] **Collision contact formal**'s & and calculate collision vector of marble and wall.
- [ ] hole and marble interaction. Determine the center of sprite(hole). If one spite touch/collide center of other event.

### Marble + Board logic and build steps

Mouse control ~ Board tilting ~ F(Marble speed + direction):
* Center in (0, 0) and radius of save zone for mouse 50 to avoid over moving
