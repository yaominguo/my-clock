import init, { set_clock, get_time } from './my_clock.js'
import Flipper from './flipper.js'
;(function () {
  var lastTime = 0
  var vendors = ['webkit', 'moz']
  for (var x = 0; x < vendors.length && !window.requestAnimationFrame; ++x) {
    window.requestAnimationFrame = window[vendors[x] + 'RequestAnimationFrame']
    window.cancelAnimationFrame = window[vendors[x] + 'CancelAnimationFrame'] || window[vendors[x] + 'CancelRequestAnimationFrame']
  }

  if (!window.requestAnimationFrame) {
    window.requestAnimationFrame = function (callback, element) {
      var currTime = new Date().getTime()
      var timeToCall = Math.max(0, 16.7 - (currTime - lastTime))
      var id = window.setTimeout(function () {
        callback(currTime + timeToCall)
      }, timeToCall)
      lastTime = currTime + timeToCall
      return id
    }
  }
  if (!window.cancelAnimationFrame) {
    window.cancelAnimationFrame = function (id) {
      clearTimeout(id)
    }
  }
})()

async function run() {
  await init()
  const flippers = []
  const flips = document.querySelectorAll('.flip')
  const time = get_time('%H%M')

  for (let i = 0; i < flips.length; i++) {
    flippers.push(
      new Flipper({
        node: flips[i],
        frontText: time[i],
        backText: +time[i] + 1,
      })
    )
  }

  setInterval(() => {
    const time = get_time('%H%M')
    for (let i = 0; i < flippers.length; i++) {
      if (flippers[i].config.frontText == time[i]) {
        continue
      }
      flippers[i].flip(flippers[i].config.frontText, time[i])
    }
  }, 1000)
}

run()
