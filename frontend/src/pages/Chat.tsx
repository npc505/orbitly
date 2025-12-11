import { useState, useEffect } from "react";
import { useNavigate, useParams, useLocation } from "react-router-dom";

interface Message {
  id: string;
  text: string;
  sender: "me" | "other";
  time: string;
}

export default function Chat() {
  const navigate = useNavigate();
  const { userId } = useParams();
  const location = useLocation();
  const { usuario, avatar } = location.state || { 
    usuario: "Usuario", 
    avatar: "https://upload.wikimedia.org/wikipedia/commons/thumb/5/59/User-avatar.svg/2048px-User-avatar.svg.png" 
  };

  // Cargar mensajes desde localStorage o usar mensajes por defecto
  const getInitialMessages = (): Message[] => {
    const saved = localStorage.getItem(`chat_${userId}`);
    if (saved) {
      return JSON.parse(saved);
    }
    // Mensajes por defecto solo la primera vez
    return [
      { id: "1", text: "Hola! ¿Cómo estás?", sender: "other", time: "10:30" },
      { id: "2", text: "¡Hola! Todo bien, ¿y tú?", sender: "me", time: "10:32" },
    ];
  };

  const [messages, setMessages] = useState<Message[]>(getInitialMessages());
  const [newMessage, setNewMessage] = useState("");

  // Guardar mensajes en localStorage cada vez que cambien
  useEffect(() => {
    localStorage.setItem(`chat_${userId}`, JSON.stringify(messages));
    
    // Actualizar el último mensaje en los chats recientes
    const lastMessage = messages[messages.length - 1];
    if (lastMessage) {
      const chatsRecientes = JSON.parse(localStorage.getItem('chatsRecientes') || '[]');
      const chatIndex = chatsRecientes.findIndex((c: any) => c.id === userId);
      
      if (chatIndex !== -1) {
        chatsRecientes[chatIndex].mensaje = lastMessage.text;
        chatsRecientes[chatIndex].time = "Ahora";
        localStorage.setItem('chatsRecientes', JSON.stringify(chatsRecientes));
      }
    }
  }, [messages, userId]);

  function handleSend() {
    if (!newMessage.trim()) return;

    const now = new Date();
    const time = `${now.getHours()}:${now.getMinutes().toString().padStart(2, '0')}`;

    const newMsg: Message = {
      id: Date.now().toString(),
      text: newMessage,
      sender: "me",
      time: time,
    };

    setMessages([...messages, newMsg]);
    setNewMessage("");
  }

  return (
    <div className="bg-gray-800 text-white w-full h-full flex flex-col">
      
      {/* Header */}
      <div className="bg-gray-900 p-4 flex items-center border-b border-gray-700">
        <button 
          onClick={() => navigate("/feed")}
          className="mr-3 text-xl hover:text-gray-400"
        >
          ← 
        </button>
        <img
          src={avatar}
          className="w-10 h-10 rounded-full bg-gray-700 object-cover mr-3"
        />
        <div>
          <p className="font-semibold">@{usuario}</p>
          <p className="text-xs text-gray-400">En línea</p>
        </div>
      </div>

      {/* Mensajes */}
      <div className="flex-1 overflow-y-auto p-4 space-y-3">
        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex ${msg.sender === "me" ? "justify-end" : "justify-start"}`}
          >
            <div
              className={`max-w-xs px-4 py-2 rounded-2xl ${
                msg.sender === "me"
                  ? "bg-indigo-500 text-white"
                  : "bg-gray-700 text-white"
              }`}
            >
              <p>{msg.text}</p>
              <p className="text-xs text-gray-300 mt-1">{msg.time}</p>
            </div>
          </div>
        ))}
      </div>

      {/* Input */}
      <div className="bg-gray-900 p-4 flex items-center border-t border-gray-700">
        <input
          type="text"
          placeholder="Escribe un mensaje..."
          value={newMessage}
          onChange={(e) => setNewMessage(e.target.value)}
          onKeyPress={(e) => e.key === "Enter" && handleSend()}
          className="flex-1 bg-gray-700 text-white px-4 py-2 rounded-full outline-none 
          focus:ring-2 focus:ring-indigo-500"
        />
        <button
          onClick={handleSend}
          className="ml-3 bg-indigo-500 px-5 py-2 rounded-full hover:bg-indigo-400 
          font-semibold"
        >
          Enviar
        </button>
      </div>
    </div>
  );
}