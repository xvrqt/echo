# Echo

Tool to generate a static micro-blog website.

## Quickstart
Download and install this program by running:
```
cargo install echo
```

Create a new Echo project, change directories inside it
```bash
echo init
cd echo
```

Create a new Echo post
```bash
echo new
```
This will open your text editor, allowing you to write your blog entry. Save the file and quit the editor when you're done.

The project should have been built, check it out in dist/index.html
You can change the title, description and author by running:
```bash
echo update <title|description|author> <value>
```

You can rebuild the project by running:
```bash
echo build
```

You can delete/update posts by running:
```bash
echo update <post_id>
echo delete <post_id>
```

You can view post ids by running:
```bash
echo log
```

