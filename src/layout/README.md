# layout specification

In `xtop`, the layout of each component can be configured by layout file in JSON format.  

This README defines the attributes in config file and explain about each attributes.


## Name

`name` field specifies the name of each component. Available name is defined in `/src/layout/config.rs`. For now, more than two same component is invalid.

## Height

Basically, the layout is a set of `Line`s stacked vertically. `line` field specifies the height of each component. The height of each `Line` is determined by the largest component in the same `Line`.  

If you want to use multiple `Line` for one component, you can use `Lines::Multiple` attribute for `lines_flag` field explained below.


- `Height::Rest` attribute means this component uses all the height remained. 
- `Height::Line` attribute tells the absolute number of lines this component uses.
- `Height::Minus` attribute is all most same with `Height::Rest` attribute, but the component leaves `n` lines of all the height remained.

## Line Attribute

As stated above, the height of `Line` is determined by the largest component in the same `Line`. It means that you can use one `Line` for one component basically.  

However, you can use multiple `Line`s by specifying `Lines::Multiple` attribute for `lines_flag` field. When this attribute is set, this component is ignored when calculating the height of `Line`.

Otherwise, you should specify `Lines::Single` attribute.

### Limitation

`Lines::Multiple` attribute is valid only for the component which is placed at left-most or right-most. In other words, multiple-`Line` component cannot be placed at center.  

When this attribute is set, width of next `Line` is calculated by substracting the width of the component.


## Width Ratio

`ratio` field determines the width of the component. The width is determined in ratio, and cannot specify the absolute columns to use.

- `Size::Ratio` attribute tells the exact ratio of width agains entire screen size.
- `Size::Rest` attribute means that the component uses all the columns remained in the `Line`.

### Limitation

You have to use `Size::Rest` attribute to go to new `Line`. In other words, you can't use `Size::Ratio(1.0)` to tell that the component should use entire `Line`. 

However for example, you can use `Size::Ratio(0.8)` for one component and place `Empty` component at right with `Size::Rest` for `ratio`. In this way, you can place the component with 80% width and left aligned.


## Note

`Line` does NOT mean a single line of ther terminal. It means multiple lines where some components are placed horizontally.
