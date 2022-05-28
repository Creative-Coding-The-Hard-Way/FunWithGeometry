initSidebarItems({"enum":[["ComposedMessage","Messages of this type are used by Composite widgets to differentiate between “external” events which are meant to pass-through to user code and “internal” events which are used to manage the widget’s state."],["Constraint","Constraints can be applied to the width and height of containers to limit their size."],["HAlignment","Represents the Horizontal Alignment for a [`Widget`] wrapped by [`Align`]."],["VAlignment","Represents the Vertical Alignment for a [`Widget`] wrapped by [`Align`]."]],"mod":[["prelude",""]],"struct":[["Align","A [`Widget`] which wraps a contiained Widget to automatically align it within the available space."],["Button","A Button is a UI widget which can fire a message when clicked."],["Col","A Col is a collection of wigets which is arranged in a single horizontal row."],["ComposedElement","An Element decorator which automatically wraps all underlying events into [‘ComposedMessage’] external values."],["Composite","This type wraps any type which implements CompositeWidget to act like a single Widget."],["Container","A generic container for another [`Widget`]. Containers have margin, padding, and a border akin to the standard CSS box model."],["Element","An Element is a type-erased widget. Elements allow UI objects to hold a variety of Widget implementations and dynamically dispatch function calls as needed."],["HSplit","A widget which splits the available area in half horizontally for a left side and right side widget."],["Label",""],["Row","A Row is a collection of wigets which is arranged in a single horizontal row."],["Slider",""],["Window","A Window is a collapsable panel with a title button which toggles the visibility of the contents."]],"trait":[["CompositeWidget","Types which implemnet this type can act like a stateful collection of widges in the UI layout."],["Widget","Widgets are UI building blocks. Widgets ar responsible for handling system events and transforming them into an instance of their own Message when the relevant sequence of events occurs."],["WithContainer","Define an extra associated method which automatically wraps any widget with a container."]]});