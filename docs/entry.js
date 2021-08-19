import init, { run } from './my_clock.js'
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

async function start() {
  await init()
  run()

  var screenBtn = document.querySelector('#screenBtn')
  var themeBtn = document.querySelector('#themeBtn')

  themeBtn.addEventListener('click', function (e) {
    e.preventDefault()
    var type = this.getAttribute('src')
    var screen = screenBtn.getAttribute('src')

    if (type === './moon.svg') {
      document.body.className = 'dark'
      this.setAttribute('src', './sun.svg')
      if (screen === './full.svg') {
        screenBtn.setAttribute('src', './full-dark.svg')
      } else if (screen === './exit.svg') {
        screenBtn.setAttribute('src', './exit-dark.svg')
      }
    } else {
      document.body.className = ''
      this.setAttribute('src', './moon.svg')
      if (screen === './full-dark.svg') {
        screenBtn.setAttribute('src', './full.svg')
      } else if (screen === './exit-dark.svg') {
        screenBtn.setAttribute('src', './exit.svg')
      }
    }
  })
  screenBtn.addEventListener('click', function (e) {
    e.preventDefault()
    var type = this.getAttribute('src')
    var className = document.body.className
    if (className === 'dark') {
      if (type === './full-dark.svg') {
        document.documentElement && document.documentElement.requestFullscreen()
        this.setAttribute('src', './exit-dark.svg')
      } else {
        document.exitFullscreen && document.exitFullscreen()
        this.setAttribute('src', './full-dark.svg')
      }
    } else {
      if (type === './full.svg') {
        document.documentElement && document.documentElement.requestFullscreen()
        this.setAttribute('src', './exit.svg')
      } else {
        document.exitFullscreen && document.exitFullscreen()
        this.setAttribute('src', './full.svg')
      }
    }
  })
}

start()
