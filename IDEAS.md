# Structure

## Engine

The Engine is Rancor's unified, opinionated store. It receives actions initiated by the DOM
and dispatches DOM update events to the global event bus. It also communicates with remote resources
and local storage.

## Controller

The Controller keeps track of the DOM hierarchy. This is the closest thing Rancor has to a shadow DOM,
but rather than tracking a complete DOM, it instead tracks a hierarchy of fragments.

When the Controller receives update events, it updates the hierarchy accordingly. For example, a router
navigate event triggers the corresponding template to become the active one.

When the Controller receives update events, it also dispatches them to existing members of the hierarchy,
so individual DOM elements can react.

## DOM

The DOM is the regular web DOM.

HTML elements interact with the Engine and Controller in the following ways:

- Selectors. An HTML element can bind one of its attributes to a selector. The controller then keeps it
  updated so it always reflects the value that the selector points to in the Engine.
- Dispatchers. DOM elements can call into the engine in their listener events.
