# iced_drop

A small library which provides a custom widget and operation to make drag and drop easier to implement in [iced](https://github.com/iced-rs/iced/tree/master)

## Usage

To add drag and drog functionality, first define two messages with the following format

```rust
enum Message {
    Drop(iced::Point, iced::Rectangle)
    HandleZones(Vec<(iced::advanced::widget::Id, iced::Rectangle)>)
}
```

The `Drop` message will be sent when the droppable is being dragged, and the left mouse button is released. This message provides the mouse position and layout boundaries of the droppable at the release point.

The `HandleZones` message will be sent after an operation that finds the drop zones under the mouse position. It provides the Id and bounds for each drop zone.

Next, create create a droppable in the view method and assign the on_drop message. The dropopable function takes an `impl Into<Element>` object, so it's easy to make a droppable from any iced widget.

```rust
iced_drop::droppable("Drop me!").on_drop(Message::Drop);
```

Next, create a "drop zone." A drop zone is any widget that operates like a container andhas some assigned Id. It's important that the widget is assigned some Id or it won't be recognized as a drop zone.

```rust
iced::widget::container("Drop zone")
    .id(iced::widget::container::Id::new("drop_zone"));
```

Finally, handle the updates of the drop messages

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

On Drop, we return a widget operation that looks for drop zones under the cursor_pos. When this operation finishes, it returns the zones found and sends the `HandleZones` message. In this example, we only defined one zone, so the zones vector will either be empty if the droppable was not dropped on the zone, or it will contain the `drop_zone`

## Examples

There are two examples: color, todo.

The color example is a very basic drag/drop showcase where the user can drag colors into zones and change the zone's color. I would start here.

[Link to video](https://drive.google.com/file/d/1K1CCi2Lc90IUyDufsvoUBZmUCbeg6_Fi/view?usp=sharing)

To run this examples: `cargo run -p color`

The todo example is a basic todo board application similar to Trello. This is a much much more complex example as it handles custom highlighting and nested droppables, but it just shows you can make some pretty cool things with iced.

[Link to video](https://drive.google.com/file/d/1MLOCk4Imd_oUnrTj_psbpYbwua976HmR/view?usp=sharing)

To run this example try: `cargo run -p todo`

Note: the todo example might also be a good example on how one can use operations. Check examples/todo/src/operation.rs. I didn't find any other examples of this in the iced repo except for the built in focus operations.

## Future Development

Right now it's a little annoying having to work with iced's Id type. At some point, I will work on a drop_zone widget that can take some generic clonable type as an id, and I will create a seperate find_zones operation that will return a list of this custom Id. This should make it easier to determine which drop zones were found.
