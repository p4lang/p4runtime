# P4Runtime specification documents

## Writing Madoko content

Please refer to the [Madoko documentation](http://madoko.org/reference.html) and
this FAQ.

### FAQ

#### How to write nested numbered lists in Madoko?

It's easy but you cannot us x.y.z. to number the nested items otherwise it won't
render properly. Instead use a single number for all items and make sure you
indent each nested level with 4 spaces:
```
1. aaa1
    1. aaa11
        1. aaa111
        2. aaa112
    2. aaa12
        1. aaa121
...
```

#### When to use Madoko tables?

Madoko tables are pretty limited. In particular, according to the documentation,
"Every row can be on one line only". Tables tend to render pretty well in HTML,
but not in PDF: by default it seems that text inside a cell never gets
wrapped. There may be a solution to this, but to avoid having to deal with Tex
directly too much, we tend to use lists instead.

#### What determines how code blocks are rendered?

Quite a few thing actually.

The `pre, code` metadata rule sets some defaults for all code blocks, such as
font family and size. See
[here](http://madoko.org/reference.html#sec-css-font-family).

We also use some CSS rules in the metadata section to customize the behavior of
the syntax highlighter. See
[here](http://madoko.org/reference.html#sec-advanced--customizing-highlight-colors).
For example:
```
.token.keyword    {
    font-weight: bold;
}
```

We use custom syntax highlighters for C++, P4, Protobuf and Protobuf text
messages. For each of these we have a JSON file (e.g. `p4.json`). They are
"imported" through the `Colorizer` metadata key:
```
Colorizer: p4
Colorizer: proto
Colorizer: prototext
Colorizer: cpp
```
The name of the language can be specified when introducing a code block:
````
```<language>
...code...
```
````
See
[here](http://madoko.org/reference.html#sec-advanced--custom-syntax-highlighting)
for more information.

Finally, to facilitate the definition of code blocks, we use [replacement
rules](http://madoko.org/reference.html#sec-replace) for each language:
```
p4example {
  replace: "~ Begin P4ExampleBlock&nl;\
                 ````p4&nl;&source;&nl;````&nl;\
                 ~ End P4ExampleBlock";
  padding:6pt;
  margin-top: 6pt;
  margin-bottom: 6pt;
  border: solid;
  background-color: #ffffdd;
  border-width: 0.5pt;
}
```
This enables us to define additional properties which will be applied to every
"p4" code block (e.g. background color). Defining a new P4 code block becomes
very easy:
```
~ Begin P4Example
header PacketOut_t {
  bit<9> egress_port;
  bit<8> queue_id;
}
~ End P4Example
```
It seems that when defining the replacement rule, the name of the rule and the
name of the block tag have to match, but the case doesn't matter (`p4example`
and `P4Example`).

## CI upload of built documents

Travis takes care of uploading the built HTML version of the spec to AWS S3. The
latest working draft (master branch) can be found
[here](https://s3-us-west-2.amazonaws.com/p4runtime/docs/master/P4Runtime-Spec.html).

Additionally, you can access the HTML version of the spec for any given branch
of this repository by using the following URL:
https://s3-us-west-2.amazonaws.com/p4runtime/travis/<your_branch_name>/P4Runtime-Spec.html.

Unfortunately, because of how Travis encrypts environment variables (which are
required to upload documents to S3), this does not work for branches in forked
repositories, even for opened pull requests.
