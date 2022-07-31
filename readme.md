# SubModel

**_The project is not completed_**

## progress

- [x] load necessary SubModel information from marco input
- [x] generate codes
- [x] field type mapping
- [x] extra sub model field
- [ ] support `Generic`
- [ ] condition field type mapping
- [ ] a better way to add extra `proc_marco` onto field on SubModels

## usage

using `all` or `none` define sub model type

- `all` default all of the field is a part of the sub model
- `none` default is an empty struct

in the `all` or `none`

- if only need define the name of sub model, can use like that

```rust
#[sub_model(all("foo"),none("foo2"))]
```

- or else using full pattern `name = "foo"`

there a a set of information can be define

- `vis` the visibility of the sub model, default `pub`
- `name` the name of the sub model
- `extra_field` the extra field of the sub model, can use like that
  - the `ty` define the type of extra field
  - the `from` define how to create the value in this field, it accept a path to a function without any params

```rust
#[all(
    name = "foo",
    extra_field(
        foo(ty = "bool", from = "Default::default")
))]
```

the example add a new field named `foo` with type `bool`,create by `Default::default` to sub model `foo`

- `extra` other marco that add to this new sub model, for example

```rust
#[all(
    name = "foo",
    extra(
        derive(Debug, Serialize, Deserialize)
))]
```

the example make the sub model `foo` can `serialize` and `deserialize`
