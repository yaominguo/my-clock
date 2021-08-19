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
  var themeBtn = document.querySelector('#theme')
  themeBtn.addEventListener('click', function (e) {
    e.preventDefault()
    var type = this.getAttribute('src')
    if (type === './moon.svg') {
      document.body.className = 'dark'
      this.setAttribute('src', './sun.svg')
    } else {
      document.body.className = ''
      this.setAttribute('src', './moon.svg')
    }
  })
}

start()
