export default class Flipper {
  constructor(config) {
    this.config = {
      node: null,
      frontText: '0',
      backText: '1',
      duration: 600,
      ...config,
    }

    this.nodeClass = {
      flip: 'flip down',
      front: 'digital front',
      back: 'digital back',
    }

    this.frontNode = this.config.node.querySelector('.front')
    this.backNode = this.config.node.querySelector('.back')

    this.isFlipping = false

    this.init()
  }

  init() {
    this.setFront(this.config.frontText)
    this.setBack(this.config.backText)
  }

  setFront(num) {
    this.config.frontText = num
    this.frontNode.setAttribute('class', `${this.nodeClass.front} number${num}`)
  }
  setBack(num) {
    this.config.backText = num
    this.backNode.setAttribute('class', `${this.nodeClass.back} number${num}`)
  }

  flip(front, back) {
    if (this.isFlipping) return
    this.isFlipping = true
    this.setFront(front)
    this.setBack(back)

    this.config.node.setAttribute('class', `${this.nodeClass.flip} go`)

    const timer = setTimeout(() => {
      this.config.node.setAttribute('class', this.nodeClass.flip)
      this.isFlipping = false
      this.setFront(back)
      clearTimeout(timer)
    }, this.config.duration)
  }
}
