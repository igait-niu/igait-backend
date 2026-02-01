import '@/pages/AssistantPage/assistant.css';
import { useState, useEffect, useCallback, useRef } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { getAuth } from "firebase/auth";
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

interface MessagePayload {
    type: 'Error' | 'Message' | 'Waiting' | 'You' | 'Typing' | 'Info' | 'Jobs';
    content: any;
}

function Assistant() {
    /* [ WebSocket Handling ] */
    const [messageHistory, setMessageHistory] = useState<MessagePayload[]>([
        { type: 'Message', content: 'Hello! How can I help you today?' },
        { type: 'Info', content: 'You can ask me about:\n* Your past submissions\n* How the iGait systems work\n* Next steps with your pre-screening' }
    ]);
    const [token, setToken] = useState('');
    const [inputMessage, setInputMessage] = useState('');
    const [waitingStatus, setWaitingStatus] = useState('');
    const [isClosed, setIsClosed] = useState(false);
    const authSent = useRef(false); // Track if auth message has been sent

    const { sendMessage, lastMessage, readyState } = useWebSocket(
        "wss://api.igaitapp.com/api/v1/assistant_proxied",
        {
            onClose: (c) => {
                console.error("WebSocket closed:");
                console.dir(c);
                //alert("The connection was closed, please refresh to re-open it!");
                setIsClosed(true);
            },
            onError: (e) => {
                console.error("WebSocket error:");
                console.dir(e);
                alert("An WebSocket error occurred. Please try again later!");
            },
            heartbeat: {
                message: 'ping',
                returnMessage: 'pong',
                timeout: 15000, // 1 minute, if no response is received, the connection will be closed
                interval: 5000, // every 25 seconds, a ping message will be sent
            },
        }
    );

    useEffect(() => {
        if (lastMessage !== null) {
            const parsedMessage: MessagePayload = JSON.parse(lastMessage.data);
            
            switch (parsedMessage.type) {
                case 'Jobs':
                    let body = "| Date | Status | Status Message | Age | Height | Weight | Sex |";
                    body  += "\n| --- | --- | --- | --- | --- | --- | --- |";

                    // For each job in `content`, add to body
                    for ( const job of parsedMessage.content ) {
                        var date = new Date(0);
                        date.setUTCSeconds(job.timestamp.secs_since_epoch);

                        body += `\n| ${date.toDateString()} | ${job.status.code} | ${job.status.value} | ${job.age} | ${job.height} | ${job.weight} | ${job.sex} |`;
                    }

                    // Add to message history
                    setMessageHistory((prev) => [...prev, { type: 'Jobs', content: body }]);
                    break;
                case 'Waiting':
                    setWaitingStatus(parsedMessage.content);
                    break;
                case 'Message':
                    // Remove typing message
                    setMessageHistory((prev) => prev.filter((msg) => msg.type !== 'Typing'));

                    setWaitingStatus('');
                    setMessageHistory((prev) => [...prev, parsedMessage]);
                    break;
                case 'Error':
                    console.error("Fatal error:", parsedMessage.content);
                    alert("An error occurred: " + parsedMessage.content);
                    break;
            }
        }
    }, [lastMessage]);

    /* [ Input Handling ] */
    function handleKeyDown ( e: any ) {
        if (e.key === 'Enter') {
            handleClickSendMessage();
        }
    }
    const handleClickSendMessage = useCallback(() => {
        if (inputMessage.trim()) {
            const typingMessage = 'Typing...';

            sendMessage(inputMessage);
            setMessageHistory((prev) => [
                ...prev,
                { type: 'You', content: inputMessage },
                { type: 'Typing', content: typingMessage }
            ]);
            setWaitingStatus('Processing your request...');
            setInputMessage(''); // Clear input after sending
        }
    }, [inputMessage, sendMessage]);
    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => setInputMessage(e.target.value);

    /* [ Google Auth ] */
    useEffect(() => {
        const auth = getAuth();
        const user = auth.currentUser;

        if (user) {
            user.getIdToken()
                .then((token) => {
                    console.log("Got token:", token);
                    setToken(token);
                })
                .catch(console.error);
        }
    }, []);

    /* [ Send Auth Message Only Once Per Connection ] */
    useEffect(() => {
        if (readyState === ReadyState.OPEN && token && !authSent.current) {
            sendMessage(token);
            authSent.current = true; // Mark as sent
        }

        // Reset when connection closes
        if (readyState === ReadyState.CLOSED) {
            authSent.current = false;
        }
    }, [token, readyState, sendMessage]);

    const renderMessages = () => {
        return messageHistory.map((msg, index) => (
            <div key={index} className={`message ${msg.type.toLowerCase()}`}>
                <Markdown remarkPlugins={[remarkGfm]}>
                    {msg.content}
                </Markdown>
            </div>
        ));
    };

    return (
        <div className="assistant-page">
            <div className="assistant-hero">
                <h1 className="assistant-hero-title">AI Assistant</h1>
                <p className="assistant-hero-subtitle">
                    Ask questions about your submissions, next steps, or how iGAIT works
                </p>
            </div>

            {isClosed && (
                <div className="connection-error">
                    <div className="error-icon">⚠️</div>
                    <h2>Connection Closed</h2>
                    <p>The connection was closed. Please refresh the page to reconnect.</p>
                    <button onClick={() => window.location.reload()} className="button-primary">
                        Refresh Page
                    </button>
                </div>
            )}

            <div className="chat-container">
                <div className="message-list">
                    {renderMessages()}
                </div>
                
                <div className="input-area">
                    {waitingStatus ? (
                        <div className="waiting-indicator">
                            <div className="loading-dots">
                                <span></span>
                                <span></span>
                                <span></span>
                            </div>
                            <span className="waiting-text">{waitingStatus}</span>
                        </div>
                    ) : null}
                    
                    <div className="input-wrapper">
                        <input
                            type="text"
                            value={inputMessage}
                            onChange={handleInputChange}
                            placeholder="Type your message..."
                            onKeyDown={handleKeyDown}
                            disabled={!!waitingStatus}
                            className="chat-input"
                        />
                        <button 
                            className='send-button' 
                            onClick={handleClickSendMessage}
                            disabled={!!waitingStatus || !inputMessage.trim()}
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" viewBox="0 0 16 16">
                                <path d="M15.854.146a.5.5 0 0 1 .11.54l-5.819 14.547a.75.75 0 0 1-1.329.124l-3.178-4.995L.643 7.184a.75.75 0 0 1 .124-1.33L15.314.037a.5.5 0 0 1 .54.11ZM6.636 10.07l2.761 4.338L14.13 2.576zm6.787-8.201L1.591 6.602l4.339 2.76z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Assistant;