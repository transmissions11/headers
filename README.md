# headers

Generate perfect code headers every time.

## Usage

```sh
λ ./headers "testing 123"
```

```sh
/*//////////////////////////////////////////////////////////////
                           TESTING 123
//////////////////////////////////////////////////////////////*/
```

It will also copy the header to your clipboard automatically.

### With VSCode

Set your global `tasks.json` like so to add the command as task:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Generate Header",
      "type": "shell",
      "command": "headers ${input:header}",
      "presentation": {
        "reveal": "never"
      }
    }
  ],
  "inputs": [
    {
      "id": "header",
      "description": "Header",
      "type": "promptString"
    }
  ]
}
```

To really speed-up your workflow, you can even add a keybind for the task in `keybindings.json`:

```json
[
  {
    "key": "CMD+h",
    "command": "workbench.action.tasks.runTask",
    "args": "Generate Header"
  }
]
```

This will copy the generate header to your clipboard.

## Credits

Inspired by virtualjpeg's [`blocky`](https://github.com/virtualjpeg/blocky).
