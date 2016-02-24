# rust-golf-lang
Golfing language written in Rust so I can learn the language more.

## docs
Each line is a function, and the last line is executed with the inputs.

```
+xy
A1x
```

With an input of `5`, this program would output `6`. Let's learn why.

`+xy` creates a function called _A_ that adds _x_ (the first argument) and _y_ (the second argument).
`A1x` creates a function called _B_ that calls _A_ with `1` and _x_ (the first argument).
The last function created is called with whatever the input is.