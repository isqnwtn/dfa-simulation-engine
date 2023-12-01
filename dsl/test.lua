MACHINE = {
  {
    name = "inactive",
    default = true,
    probs = {playing = 1.0}
  },
  {
    name = "playing",
    probs = {inactive = 0.2,paused = 0.4, stopped= 0.4}
  },
  {
    name = "paused",
    probs = {playing = 1.0}
  },
  {
    name = "stopped",
    probs = {},
  }
}