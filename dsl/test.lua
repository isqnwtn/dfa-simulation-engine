MACHINE = {
  {
    name = "inactive",
    default = true,
    probs = {playing = 1.0},
    waitTime = 90,
  },
  {
    name = "playing",
    probs = {inactive = 0.2,paused = 0.7, stopped= 0.1},
    waitTime = 200,
    waitSpread = 50,
  },
  {
    name = "paused",
    probs = {playing = 1.0},
    waitTime = 50,
    waitSpread = 25,
  },
  {
    name = "stopped",
    probs = {},
    final = true,
  }
}
