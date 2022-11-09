import init, { State, Keycode, MouseButton } from "./dist/index.js"

export async function run() {
  const wasm = await init()
  const state = State.new()

  const canvas = document.getElementById("screen")
  canvas.width = state.screen_width()
  canvas.height = state.screen_height()
  const context = canvas.getContext("2d")

  const renderLoop = () => {
    state.frame()
    render(wasm.memory.buffer, state, context)
    requestAnimationFrame(renderLoop)
  }
  requestAnimationFrame(renderLoop)

  document.addEventListener("keydown", (event) => {
    const keycode = toKeycode(event.key)
    if (keycode !== undefined) {
      event.preventDefault()
      state.key_down(keycode)
    }
  })
  document.addEventListener("keyup", (event) => {
    const keycode = toKeycode(event.key)
    if (keycode !== undefined) {
      event.preventDefault()
      state.key_up(keycode)
    }
  })
  canvas.addEventListener("mousemove", (event) => {
    state.mouse_move(
      (event.offsetX / canvas.clientWidth) * state.screen_width(),
      (event.offsetY / canvas.clientHeight) * state.screen_height()
    )
  })
  canvas.addEventListener("mousedown", (event) => {
    if (event.button === 0) {
      state.mouse_down(MouseButton.Left)
    }
    if (event.button === 2) {
      state.mouse_down(MouseButton.Right)
    }
  })
  canvas.addEventListener("mouseup", (event) => {
    if (event.button === 0) {
      state.mouse_up(MouseButton.Left)
    }
    if (event.button === 2) {
      state.mouse_up(MouseButton.Right)
    }
  })
}

function render(buffer, state, context) {
  const width = state.screen_width()
  const height = state.screen_height()
  const screen = state.screen()
  const data = new Uint8ClampedArray(buffer, screen, width * height * 4)
  context.putImageData(new ImageData(data, width, height), 0, 0)
}

function toKeycode(key) {
  switch (key) {
    case "Escape":
      return Keycode.Escape
    case "Backspace":
      return Keycode.Backspace
    case "Enter":
      return Keycode.Return
    case "ArrowLeft":
      return Keycode.Left
    case "ArrowUp":
      return Keycode.Up
    case "ArrowRight":
      return Keycode.Right
    case "ArrowDown":
      return Keycode.Down
    case "1":
      return Keycode.Num1
    case "2":
      return Keycode.Num2
    case "a":
      return Keycode.A
    case "c":
      return Keycode.C
    case "e":
      return Keycode.E
    case "q":
      return Keycode.Q
    case "s":
      return Keycode.S
    case "w":
      return Keycode.W
    case "x":
      return Keycode.X
    case "y":
      return Keycode.Y
    case "z":
      return Keycode.Z
    case "F1":
      return Keycode.F1
    case "F2":
      return Keycode.F2
    case "F3":
      return Keycode.F3
    case "F4":
      return Keycode.F4
    case "F6":
      return Keycode.F6
    case "F7":
      return Keycode.F7
    case "F8":
      return Keycode.F8
    case "F9":
      return Keycode.F9
    case " ":
      return Keycode.Space
    case "+":
      return Keycode.Plus
    case "-":
      return Keycode.Minus
    default:
      return undefined
  }
}
