<!-- /src/App.vue -->

<template>
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
      src="/assets/track-wEqzUYbv.flac"
      @canplay="onCanPlay"
      @play="isPlaying = true"
      @pause="isPlaying = false"
      @ended="onAudioEnded"
      @error="onAudioError"
      style="display: none"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { Play, Pause } from 'lucide-vue-next'

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
    console.warn('> Failed to play audio:', err)
  }
}

const onCanPlay = async () => {
  if (hasStartedLoading.value && !isPlaying.value) {
    try {
      await audioRef.value?.play()
    } catch (err) {
      console.warn('> Failed to play audio:', err)
    }
  }
}

const onAudioError = (e: Event) => {
  console.error('> Audio error:', e)
  hasStartedLoading.value = false
}

const onAudioEnded = () => {
  isPlaying.value = false
}

onUnmounted(() => {})
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
.content {
  max-width: 600px;
  padding: 2rem;
  box-sizing: border-box;
  color: var(--text-color);
  font-size: 0.875rem;
  line-height: 2.1;
  font-weight: 600;
  max-height: 100dvh;
  overflow-y: auto;
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
</style>
