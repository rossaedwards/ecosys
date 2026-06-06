"""
Memoree — LM Studio Hook
Auto-saves conversations from LM Studio models to Memoree
Works with liquid/lfm2.5-1.2b, nemotron, qwen, etc.
"""

import httpx
from schemas import EpisodicMemory

_MEMOREE_URL = "http://127.0.0.1:7042"
_LMS_URL = "http://localhost:1234/v1/chat/completions"


def _save_to_memoree(llm: str, prompt: str, response: str, project: str = "memoree"):
    """Save conversation as episodic memory"""
    try:
        memory = EpisodicMemory(
            role="assistant",
            content=f"[USER] {prompt}\n\n[ASSISTANT] {response}",
            llm=llm,
            session_id="lmstudio-default",
            tags=["lmstudio", "auto-save"],
            user_preferences={"project": project}
        )
        
        httpx.post(
            f"{_MEMOREE_URL}/memories/events",
            json=memory.model_dump(),
            timeout=5,
        )
    except Exception as e:
        print(f"[Memoree Hook] Failed to save: {e}")


def chat(model: str, prompt: str, system: str = "", project: str = "memoree", **kwargs) -> str:
    """Chat with any LM Studio model and auto-save to Memoree"""
    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})

    body = {
        "model": model,
        "messages": messages,
        "stream": False,
        **kwargs
    }

    try:
        resp = httpx.post(_LMS_URL, json=body, timeout=120)
        resp.raise_for_status()
        text = resp.json()["choices"][0]["message"]["content"]

        # Auto-save to Memoree
        _save_to_memoree(model, prompt, text, project)
        
        return text
    except Exception as e:
        print(f"[LM Studio] Error: {e}")
        return f"Error communicating with LM Studio: {e}"


# Convenience wrappers using your aliases
def liquid(prompt: str, **kwargs) -> str:
    return chat("liquid/lfm2.5-1.2b", prompt, **kwargs)

def nemotron(prompt: str, **kwargs) -> str:
    return chat("nemotron-3-nano-4b", prompt, **kwargs)

def qwen(prompt: str, **kwargs) -> str:
    return chat("qwen-3.5-4b", prompt, **kwargs)