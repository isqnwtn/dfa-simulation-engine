-- First test example for simulating a video player

function SESSION_MANAGER(t)
  if 0 < t and t < 1000 then
      return 2
   elseif 1000 < t and t < 2000 then
      return 5
    else
      return 1
   end
end

GLOBAL = {
  max_sessions = 3,
  run_length = 100,
  hb_interval = 30,

}

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
