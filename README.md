# mdtempl

Simple markdown templating.

# Usage
Put a relative path to the file you want to include in `{/ <path> /}`. And run run `mdtempl your_file.md`

### Example tree:
```tree
.
├── head_includes
│   └── include.md
├── heads
│   └── head1.md
├── test.md
└── test_levels
    └── test.md
```
### File contents:
```md
<!-- test_levels/test.md -->
{/ ../heads/head1 /}

# Test Levels
```
```md
<!-- heads/head1.md -->
# Head1

{/ ../head_includes/include /}
```
```md
<!-- head_includes/include.md -->
# Include
```

### Output
run `mdtempl test_md/test_levels/test.md`

Output:
```md
# Head1

# Include

# Test Levels
```