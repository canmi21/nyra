<!-- /ui/App.vue -->

<template>
  <div class="container">
    <div class="dots-container" ref="dotsContainer">
      <div
        v-for="dot in dots"
        :key="dot.id"
        class="dot"
        :style="{
          left: dot.x + 'px',
          top: dot.y + 'px',
          opacity: dot.opacity,
          transform: `scale(${dot.scale})`,
          animationDuration: dot.animationDuration + 's',
          animationDelay: dot.animationDelay + 's',
        }"
      ></div>
    </div>

    <div class="content">
      <div class="line-with-button">
        <button @click="handlePlayClick" class="play-btn">
          <Play v-if="!isPlaying" :size="16" fill="currentColor" />
          <Pause v-else :size="16" fill="currentColor" />
        </button>
        <p class="line">
          <span
            v-for="(char, charIndex) in lineChars(lines[0])"
            :key="charIndex"
            class="char"
            :style="{
              animationDelay: `${lineStartTimes[0] + charIndex * charDelay}s`,
            }"
          >
            {{ char }}
          </span>
        </p>
      </div>

      <p v-for="(line, lineIndex) in lines.slice(1)" :key="lineIndex + 1" class="line indented">
        <span
          v-for="(char, charIndex) in lineChars(line)"
          :key="charIndex"
          class="char"
          :style="{
            animationDelay: `${lineStartTimes[lineIndex + 1] + charIndex * charDelay}s`,
          }"
        >
          {{ char }}
        </span>
      </p>

      <audio
        ref="audioRef"
        preload="none"
        src="https://static.canmi.icu/track-wEqzUYbv.flac"
        @canplay="onCanPlay"
        @play="isPlaying = true"
        @pause="isPlaying = false"
        @ended="onAudioEnded"
        @error="onAudioError"
        @loadedmetadata="onLoadedMetadata"
        style="display: none"
      />
    </div>

    <footer class="footer">
      <a href="https://beian.miit.gov.cn/" target="_blank" rel="noopener noreferrer">
        沪ICP备 20028632 号
      </a>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { Play, Pause } from 'lucide-vue-next'

interface Dot {
  id: number
  x: number
  y: number
  vx: number
  vy: number
  opacity: number
  scale: number
  life: number
  maxLife: number
  animationDuration: number
  animationDelay: number
}

const lyrics = `ずっと君の傍にいるよ
どんな未来が僕らを試したって きっと
ふたりの運命 めぐり逢えたのは
君の声 聴こえたから
そう ピンチな出来事 押し寄せて来ても
君といれば 乗り越えられる
ときめきと 負けん気と
裏腹でハラハラするけど
ただ 君を守りたいよ
遠い世界で生まれたふたりだけど
ずっと君の傍にいるよ
どんな未来も希望に変えよう`

const lines = lyrics.split('\n')
const charDelay = 0.06
const lineEndDelay = 0.3

const lineStartTimes = lines.reduce((acc, _, i) => {
  if (i === 0) {
    acc.push(0)
  } else {
    const prevLine = lines[i - 1]
    const prevDuration = prevLine.length * charDelay
    const prevStart = acc[i - 1]
    acc.push(prevStart + prevDuration + lineEndDelay)
  }
  return acc
}, [] as number[])

const lineChars = (line: string) => [...line]

const audioRef = ref<HTMLAudioElement | null>(null)
const isPlaying = ref(false)
const hasStartedLoading = ref(false)
const dotsContainer = ref<HTMLElement | null>(null)
const dots = ref<Dot[]>([])
const maxDots = 25
const dotIdCounter = ref(0)
let animationFrameId: number | null = null
let containerWidth = 0
let containerHeight = 0

const createDot = (): Dot => {
  return {
    id: dotIdCounter.value++,
    x: Math.random() * containerWidth,
    y: Math.random() * containerHeight,
    vx: (Math.random() - 0.5) * 0.25,
    vy: (Math.random() - 0.5) * 0.5,
    opacity: Math.random() * 0.6 + 0.2,
    scale: Math.random() * 0.8 + 0.4,
    life: 0,
    maxLife: Math.random() * 8000 + 4000,
    animationDuration: Math.random() * 3 + 2,
    animationDelay: Math.random() * 2,
  }
}

const initDots = () => {
  if (!dotsContainer.value) return

  const rect = dotsContainer.value.getBoundingClientRect()
  containerWidth = rect.width
  containerHeight = rect.height

  let currentDotCount = 0
  const addDotsGradually = () => {
    if (currentDotCount < maxDots) {
      dots.value.push(createDot())
      currentDotCount++
      setTimeout(addDotsGradually, Math.random() * 300 + 100) // 100-400ms
    }
  }

  addDotsGradually()
}

const updateDots = () => {
  dots.value = dots.value.filter((dot) => {
    dot.x += dot.vx
    dot.y += dot.vy
    dot.life += 16

    const isOutOfBounds =
      dot.x < -10 || dot.x > containerWidth + 10 || dot.y < -10 || dot.y > containerHeight + 10

    const isLifeExpired = dot.life > dot.maxLife

    return !isOutOfBounds && !isLifeExpired
  })

  while (dots.value.length < maxDots && Math.random() < 0.02) {
    dots.value.push(createDot())
  }

  animationFrameId = requestAnimationFrame(updateDots)
}

const handlePlayClick = async () => {
  if (!audioRef.value) return

  try {
    if (isPlaying.value) {
      audioRef.value.pause()
    } else {
      if (!hasStartedLoading.value) {
        hasStartedLoading.value = true
        audioRef.value.load()
      } else {
        await audioRef.value.play()
      }
    }
  } catch (err) {
    console.warn('Failed to play audio:', err)
  }
}

const onCanPlay = async () => {
  if (hasStartedLoading.value && !isPlaying.value) {
    try {
      await audioRef.value?.play()
    } catch (err) {
      console.warn('Failed to play audio:', err)
    }
  }
}

const onLoadedMetadata = () => {
  if (audioRef.value) {
    audioRef.value.volume = 0.3
  }
}

const onAudioError = (e: Event) => {
  console.error('Audio error:', e)
  hasStartedLoading.value = false
}

const onAudioEnded = () => {
  isPlaying.value = false
}

const handleResize = () => {
  if (!dotsContainer.value) return
  const rect = dotsContainer.value.getBoundingClientRect()
  containerWidth = rect.width
  containerHeight = rect.height
}

onMounted(async () => {
  if (audioRef.value) {
    audioRef.value.volume = 0.3
  }

  await nextTick()
  initDots()
  updateDots()

  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId)
  }
  window.removeEventListener('resize', handleResize)
})
</script>

<style>
#app {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>

<style scoped>
.container {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}

.dots-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 1;
}

.dot {
  position: absolute;
  width: 2px;
  height: 2px;
  background-color: rgba(255, 255, 255, 0.6);
  border-radius: 50%;
  animation: twinkle infinite ease-in-out alternate;
  will-change: transform, opacity;
}

@keyframes twinkle {
  0% {
    opacity: 0.2;
    transform: scale(0.8);
  }
  100% {
    opacity: 0.8;
    transform: scale(1.2);
  }
}

.content {
  position: relative;
  z-index: 2;
  max-width: 600px;
  padding: 2rem;
  box-sizing: border-box;
  color: var(--text-color);
  font-size: 0.875rem;
  line-height: 2.1;
  font-weight: 600;
  max-height: 100dvh;
  overflow-y: auto;
  padding-bottom: 4rem;
}

.line-with-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.play-btn {
  border: none;
  background: transparent;
  color: var(--text-color);
  cursor: pointer;
  opacity: 0.5;
  transition: opacity 0.2s ease;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.play-btn:hover {
  opacity: 1;
}

.line {
  margin: 0;
  padding: 0;
  white-space: pre-wrap;
  word-break: break-word;
  text-align: left;
}

.line.indented {
  padding-left: calc(16px + 0.5rem);
}

.char {
  display: inline-block;
  opacity: 0;
  transform: translateY(0);
  animation-name: floatUp;
  animation-duration: 0.5s;
  animation-fill-mode: forwards;
  animation-timing-function: ease-out;
  will-change: transform, opacity;
}

@keyframes floatUp {
  0% {
    opacity: 0;
    transform: translateY(15px);
  }
  50% {
    transform: translateY(-5px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.footer {
  position: absolute;
  bottom: 1.5rem;
  left: 0;
  width: 100%;
  text-align: center;
  font-size: 0.75rem;
  color: var(--text-color);
  opacity: 0.6;
  pointer-events: none;
  z-index: 2;
}

.footer a {
  color: inherit;
  text-decoration: none;
  transition: opacity 0.2s ease;
  pointer-events: auto;
}

.footer a:hover {
  opacity: 1;
}
</style>
