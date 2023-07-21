import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import List from './List'

const wasm = await import('papers')

function App() {
  const [count, setCount] = useState(0)

  return (<div>
    <div>{wasm.list_component()}</div>
    
  </div>
  )
}

export default App
