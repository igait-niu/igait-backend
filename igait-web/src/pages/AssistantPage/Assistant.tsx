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
        <div className="Assistant">
            <div className='about-header'>
            </div>
            <div className='about-header'>
            </div>
            {isClosed && (
                <div className="error-message">
                    <h2>Connection Closed</h2>
                    <p>The connection was closed. Please refresh the page to re-open it.</p>
                </div>
            )}
            <div className="message-container">
                {renderMessages()}
            </div>
            <div className="input-container">
                {waitingStatus ? (
                    <input
                        type = "text"
                        value = {waitingStatus}
                        onChange = {handleInputChange}
                        placeholder = "Type your message..."
                        disabled
                    />
                    ) : (
                    <input
                        type="text"
                        value={inputMessage}
                        onChange={handleInputChange}
                        placeholder="Type your message..."
                        onKeyDown={handleKeyDown}
                    />
                )}
                {!waitingStatus &&
                    <button className='assistant-submit' onClick={handleClickSendMessage}>Send</button>
                }
            </div>
        </div>
    );
}

export default Assistant;