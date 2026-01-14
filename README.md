# iced_drop

A small library which provides a custom widget and operation to make drag and drop easier to implement in [iced](https://github.com/iced-rs/iced/tree/master)

## Usage

1. To start implementing drag and drop functionality, first define two messages with the parameter specifications below: 
* a drop message with parameters: `iced::Point`, `iced::Rectangle`
* a handler message with parameter: `Vec<(iced::advanced::widget::Id, iced::Rectangle)>`

```rust
enum Message {
    Drop(iced::Point, iced::Rectangle),
    HandleZones(Vec<(iced::advanced::widget::Id, iced::Rectangle)>)
}
```

- The `Drop` message will be published when the left mouse button is released if the widget was being dragged (left click + mouse movement). This message provides the mouse position and layout boundaries of the droppable at the release point.
- The `HandleZones` message will be published on completion of the `iced_drop::zones_on_point` operation which finds the drop zones under the mouse position. It provides the ID and bounds for each drop zone under the given mouse position.
- The general idea is that one can use the arguments of `Drop` to feed into `zones_on_point` to get a `HandleZones` message which gives any information necessary to handle general drag-drop implementation.

2. Next, create a droppable in the view method and assign the on_drop message. The droppable function takes an `impl Into<Element>` object, so it's easy to make a droppable from any iced widget.

```rust
iced_drop::droppable("Drop me!").on_drop(Message::Drop);
```

3. Next, create a "drop zone." A drop zone is any widget that operates like a container and has some assigned ID. It's important that the widget is assigned some ID or it won't be recognized as a drop zone.

```rust
iced::widget::container("Drop zone")
    .id(iced::widget::container::Id::new("drop_zone"));
```

4. Finally, handle the update logic using the `iced_drop::zones_on_point` operation and your drop and handler messages

```rust
match message {
    Message::Drop(cursor_pos, _) => {
        return iced_drop::zones_on_point(
            Message::HandleZonesFound,
            point,
            None,
            None,
        );
    }
    Message::HandleZones(zones) => {
        println!("{:?}", zones)
    }
}
```
In this example, we only defined one zone, so the zones vector will either be empty if the droppable was not dropped on the zone, or it will contain the `drop_zone` created on step 3

## Examples

There are two examples: color, todo.

The color example is a very basic drag/drop showcase where the user can drag colors into zones and change the zone's color. I would start here.

[Link to video](https://drive.google.com/file/d/1K1CCi2Lc90IUyDufsvoUBZmUCbeg6_Fi/view?usp=sharing)

To run this examples: `cargo run -p color`

The todo example is a basic todo board application similar to Trello. This is a **MUCH** more complex example as it handles custom highlighting and nested droppable, but it just shows you can make some pretty cool things with iced.

[Link to video](https://drive.google.com/file/d/1MLOCk4Imd_oUnrTj_psbpYbwua976HmR/view?usp=sharing)

To run this example try: `cargo run -p todo`

Note: the todo example might also be a good example on how one can use operations. Check examples/todo/src/operation.rs. I didn't find any other examples of this in the iced repo except for the built-in focus operations.

## Future Development

Right now it's a little annoying having to work with iced's Id type. At some point, I will work on a drop_zone widget that can take some generic clonable type as an id, and I will create a separate find_zones operation that will return a list of this custom Id. This should make it easier to determine which drop zones were found.

## Used in the Wild

Iced is still evolving, and part of the fun is seeing how others use it. If iced_drop shows up anywhere in your work, I’d love to link it here so others can explore, learn, and connect.
