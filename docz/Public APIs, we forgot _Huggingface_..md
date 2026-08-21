<img src="media/image-84096a2ba9f8bf695ed806fbbd9bbf3b3a89f211.png" style="width:2.64667in;height:0.66667in" />

**Public APIs, we forgot "Huggingface".**

Here's a complete **updated multi-vendor Next.js backend API route** that includes Hugging Face along with Gemini, Grok, Claude, Microsoft Copilot, DeepSeek, OpenAI, and Perplexity AI.

**1. Add Hugging Face API key to .env.local**

HUGGINGFACE_API_KEY=hf_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

**2. Updated backend API route supporting Hugging Face and other vendors**

import type { NextApiRequest, NextApiResponse } from 'next';\
\
async function callOpenAI(prompt: string): Promise\<string\> {\
const resp = await fetch('https://api.openai.com/v1/chat/completions', {\
method: 'POST',\
headers: {\
'Authorization': \`Bearer \${process.env.OPENAI_API_KEY}\`,\
'Content-Type': 'application/json',\
},\
body: JSON.stringify({\
model: 'gpt-4',\
messages: \[{ role: 'user', content: prompt }\],\
max_tokens: 512,\
}),\
});\
const data = await resp.json();\
if (!resp.ok) throw new Error(data.error?.message \|\| 'OpenAI error');\
return data.choices?.\[^0\]?.message?.content \|\| '';\
}\
\
async function callAnthropic(prompt: string): Promise\<string\> {\
const resp = await fetch('https://api.anthropic.com/v1/messages', {\
method: 'POST',\
headers: {\
'x-api-key': process.env.ANTHROPIC_API_KEY!,\
'Content-Type': 'application/json',\
'anthropic-version': '2023-06-01',\
},\
body: JSON.stringify({\
model: 'claude-3-opus-20240229',\
messages: \[{ role: 'user', content: prompt }\],\
max_tokens: 512,\
}),\
});\
const data = await resp.json();\
if (!resp.ok) throw new Error(data.error?.message \|\| 'Anthropic error');\
if (Array.isArray(data.content)) {\
return data.content.map((chunk: any) =\> chunk.text).join('\n');\
}\
return data.content \|\| '';\
}\
\
async function callGemini(prompt: string): Promise\<string\> {\
const resp = await fetch(\`https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=\${process.env.GOOGLE_GEMINI_API_KEY}\`, {\
method: 'POST',\
headers: { 'Content-Type': 'application/json' },\
body: JSON.stringify({ contents: \[{ parts: \[{ text: prompt }\] }\] }),\
});\
const data = await resp.json();\
if (!resp.ok) throw new Error(data.error?.message \|\| 'Gemini error');\
return data.candidates?.\[^0\]?.content?.parts?.?.text \|\| '';\
}\
\
async function callGrok(prompt: string): Promise\<string\> {\
// Placeholder for Grok API implementation\
// Replace with real API call when available\
return \`Grok response for: \${prompt}\`;\
}\
\
async function callMicrosoftCopilot(prompt: string): Promise\<string\> {\
// Placeholder for Microsoft Copilot API implementation\
// Replace with real API call when available\
return \`Microsoft Copilot response for: \${prompt}\`;\
}\
\
async function callDeepSeek(prompt: string): Promise\<string\> {\
// Placeholder for DeepSeek API implementation\
// Replace with real API call when available\
return \`DeepSeek response for: \${prompt}\`;\
}\
\
async function callPerplexity(prompt: string): Promise\<string\> {\
// Placeholder for Perplexity AI API implementation\
// Replace with real API call when available\
return \`Perplexity AI response for: \${prompt}\`;\
}\
\
// Hugging Face Inference API example\
async function callHuggingFace(prompt: string): Promise\<string\> {\
const resp = await fetch('https://api-inference.huggingface.co/models/gpt2', {\
method: 'POST',\
headers: {\
'Authorization': \`Bearer \${process.env.HUGGINGFACE_API_KEY}\`,\
'Content-Type': 'application/json',\
},\
body: JSON.stringify({\
inputs: prompt,\
options: { wait_for_model: true }\
}),\
});\
const data = await resp.json();\
if (!resp.ok) throw new Error(data.error \|\| 'Hugging Face error');\
if (Array.isArray(data) && data.length \> 0 && data\[^0\].generated_text) {\
return data.generated_text;\
}\
return JSON.stringify(data);\
}\
\
export default async function handler(req: NextApiRequest, res: NextApiResponse) {\
if (req.method !== 'POST') {\
return res.status(405).json({ error: 'Method not allowed. Use POST.' });\
}\
\
const { prompt, vendors } = req.body;\
\
if (!prompt \|\| !Array.isArray(vendors) \|\| vendors.length === 0) {\
return res.status(400).json({ error: 'Prompt and vendors array required' });\
}\
\
try {\
const results = await Promise.all(\
vendors.map(async (vendor: string) =\> {\
try {\
const lowerVendor = vendor.toLowerCase();\
let output = '';\
switch (lowerVendor) {\
case 'openai':\
output = await callOpenAI(prompt);\
break;\
case 'anthropic':\
case 'claude':\
output = await callAnthropic(prompt);\
break;\
case 'gemini':\
output = await callGemini(prompt);\
break;\
case 'grok':\
output = await callGrok(prompt);\
break;\
case 'microsoftcopilot':\
case 'copilot':\
output = await callMicrosoftCopilot(prompt);\
break;\
case 'deepseek':\
output = await callDeepSeek(prompt);\
break;\
case 'perplexity':\
case 'me':\
output = await callPerplexity(prompt);\
break;\
case 'huggingface':\
output = await callHuggingFace(prompt);\
break;\
default:\
return { vendor, error: 'Unknown vendor' };\
}\
return { vendor, output };\
} catch (error: any) {\
return { vendor, error: error.message \|\| 'API call error' };\
}\
})\
);\
\
const responseObj: Record\<string, string\> = {};\
results.forEach((r) =\> {\
responseObj\[r.vendor\] = r.output \|\| \`Error: \${r.error}\`;\
});\
\
return res.status(200).json({ responses: responseObj });\
} catch (error: any) {\
return res.status(500).json({ error: error.message \|\| 'Server error' });\
}\
}

**Notes**

- The Hugging Face example calls the GPT-2 text generation model endpoint. You can swap the model URI (https://api-inference.huggingface.co/models/gpt2) with others available in Hugging Face Hub.

- Grok, Microsoft Copilot, DeepSeek, and Perplexity are placeholders; update those API calls when you have their official docs.

- Be sure your environment variables for API keys are correctly set and your hosting environment can read .env.local.

If you want me to help implement any specific vendor API properly or generate frontend code for calling this real backend, just ask!

⁂

1.  Claude_g0dm0d3-core-src-app-layout.pdf
