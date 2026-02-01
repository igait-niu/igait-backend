import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './routes/AppRoutes.tsx'
import { BrowserRouter } from 'react-router-dom'
import { ToastProvider } from './components/Toast.tsx'
import './styles/design-system.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <ToastProvider>
        <App />
      </ToastProvider>
    </BrowserRouter>
  </React.StrictMode>,
)