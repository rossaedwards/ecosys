<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Terms of Service — Aurphyx LLC</title>
    <meta name="description" content="Aurphyx Terms of Service - The legal framework and user agreements for interacting with our sovereign computing infrastructure.">
        
    <!-- App Icons and Manifest -->
    <link rel="icon" type="image/x-icon" href="/images/favicon.ico">
    <link rel="manifest" href="/manifest.json">
    
    <!-- OpenDyslexic Font -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/open-dyslexic@0.2.1/OpenDyslexic.min.css">
    
    <!-- Global Fonts -->
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;800&family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">

    <style>
        /* =========================================================
           GLOBAL SPA & THEME VARIABLES
           ========================================================= */
        :root { 
            --bg-main: #232946; 
            --bg-section: #22223b; 
            --text-color: #f2f2f2; 
            --text-muted: #a0aabf;
            --accent: #8be9fd; 
            --accent-hover: #2dd4bf;
            --card-bg: linear-gradient(135deg, #232946 0%, rgba(57, 62, 70, 0.5) 100%);
            --border-color: rgba(139, 233, 253, 0.15);
            --badge-bg: rgba(0, 0, 0, 0.2);
        }
        
        body.light-mode { 
            --bg-main: #f8f9fa; 
            --bg-section: #ffffff; 
            --text-color: #22223b; 
            --text-muted: #4a5568;
            --accent: #667eea; 
            --accent-hover: #5a67d8;
            --card-bg: #f1f5f9;
            --border-color: rgba(0, 0, 0, 0.1);
            --badge-bg: #ffffff;
        }
        
        body.dyslexic-font * { 
            font-family: "OpenDyslexic", "Segoe UI", Tahoma, Geneva, Verdana, sans-serif !important; 
        }

        * { margin:0; padding:0; box-sizing:border-box; }
        
        body { 
            font-family: 'Inter', "Segoe UI", Tahoma, Geneva, Verdana, sans-serif; 
            line-height: 1.7; 
            color: var(--text-color); 
            background: linear-gradient(135deg, var(--bg-main) 0%, #121629 100%); 
            overflow-x: hidden;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }

        body.light-mode {
            background: var(--bg-main);
        }

        /* =========================================================
           HEADER & NAVIGATION
           ========================================================= */
        header { 
            background: rgba(20, 20, 34, 0.97); 
            backdrop-filter: blur(10px); 
            position: fixed; top: 0; left: 0; right: 0; 
            z-index: 2000; 
            box-shadow: 0 2px 20px rgba(0,0,0,0.5); 
        }
        
        body.light-mode header {
            background: rgba(255, 255, 255, 0.97);
            box-shadow: 0 2px 20px rgba(0,0,0,0.05);
        }
        
        nav { 
            display: flex; justify-content: space-between; align-items: center; 
            max-width: 1400px; margin: 0 auto; padding: 1rem 20px; 
        }
        
        .logo { 
            font-size: 1.8rem; font-weight: bold; 
            background: linear-gradient(45deg, #FF007F, #00F0FF); 
            -webkit-background-clip: text; background-clip: text; -webkit-text-fill-color: transparent; 
            text-decoration: none; cursor: pointer;
        }
        
        .nav-links { display: flex; list-style: none; gap: 2rem; align-items: center; }
        .nav-links a { color: #f2f2f2; text-decoration: none; font-weight: 500; cursor: pointer; transition: color 0.3s; }
        .nav-links a:hover, .nav-links a.active { color: #00F0FF; }
        
        body.light-mode .nav-links a { color: #22223b; }
        body.light-mode .nav-links a:hover, body.light-mode .nav-links a.active { color: var(--accent); }

        .dropdown { position: relative; }
        .dropdown-content { 
            display: none; position: absolute; 
            background: rgba(20, 20, 34, 0.98); min-width: 160px; 
            box-shadow: 0 8px 16px rgba(0,0,0,0.3); border-radius: 6px; 
            top: 100%; left: 50%; transform: translateX(-50%); z-index: 2000; 
        }
        body.light-mode .dropdown-content { background: #ffffff; box-shadow: 0 8px 16px rgba(0,0,0,0.1); border: 1px solid #eee; }
        .dropdown-content a { display: block; padding: 10px 15px; color: #f2f2f2; }
        body.light-mode .dropdown-content a { color: #22223b; }
        .dropdown-content a:hover { background: rgba(255,255,255,0.1); }
        body.light-mode .dropdown-content a:hover { background: #f8f9fa; }
        .dropdown:hover .dropdown-content { display: block; }
        
        .header-controls { display: flex; gap: 8px; }
        .control-btn { 
            background: rgba(255,255,255,0.1); color: #f2f2f2; 
            border: none; width: 42px; height: 42px; border-radius: 50%; 
            display: flex; align-items: center; justify-content: center; 
            font-size: 1.2rem; cursor: pointer; transition: all 0.3s;
        }
        body.light-mode .control-btn { background: #f1f5f9; color: #22223b; }
        .control-btn:hover { background: #00F0FF; color: #232946; }
        body.light-mode .control-btn:hover { background: var(--accent); color: #fff; }
        
        .hamburger { display: none; flex-direction: column; cursor: pointer; gap: 4px; }
        .hamburger span { width: 25px; height: 3px; background: var(--text-color); }

        /* =========================================================
           DOCUMENT STYLES
           ========================================================= */
        main#content { 
            flex: 1;
            padding-top: 100px; 
            padding-bottom: 6rem;
            max-width: 900px;
            margin: 0 auto;
            width: 100%;
            padding-left: 20px;
            padding-right: 20px;
        }

        .document-container {
            background: var(--bg-section);
            padding: 3rem;
            border-radius: 12px;
            box-shadow: 0 16px 34px rgba(0, 0, 0, 0.2);
            border: 1px solid var(--border-color);
        }

        .doc-header {
            text-align: center;
            margin-bottom: 3rem;
            padding-bottom: 2rem;
            border-bottom: 2px solid var(--accent);
        }

        h1 { font-size: 2.5rem; margin-bottom: 0.5rem; color: var(--text-color); letter-spacing: -0.5px; }
        h2 { color: var(--accent); font-size: 1.8rem; margin-top: 2.5rem; margin-bottom: 1rem; padding-bottom: 0.5rem; border-bottom: 1px solid var(--border-color); }
        h3 { color: var(--text-color); font-size: 1.3rem; margin-top: 1.5rem; margin-bottom: 0.5rem; }
        h4 { color: var(--text-muted); font-size: 1.1rem; margin-top: 1rem; margin-bottom: 0.5rem; }
        
        p { margin-bottom: 1rem; text-align: justify; color: var(--text-color); }
        ul, ol { margin-left: 2rem; margin-bottom: 1.5rem; color: var(--text-color); }
        li { margin-bottom: 0.5rem; }

        .last-updated { font-family: 'JetBrains Mono', monospace; font-size: 0.9rem; color: var(--text-muted); }

        /* Callout Badges */
        .highlight {
            background: var(--card-bg);
            padding: 1.5rem;
            border-left: 4px solid var(--accent);
            margin: 1.5rem 0;
            border-radius: 0 8px 8px 0;
            border: 1px solid var(--border-color);
        }

        .important {
            background: rgba(255, 85, 85, 0.1);
            border-left: 4px solid #ff5555;
            color: var(--text-color);
            padding: 1.5rem;
            margin: 1.5rem 0;
            border-radius: 0 8px 8px 0;
        }

        .contact-info {
            background: var(--card-bg);
            padding: 2rem;
            border-radius: 12px;
            margin: 2rem 0;
            border: 1px solid var(--border-color);
        }

        /* Legal specific styles */
        .legal-caps {
            text-transform: uppercase;
            font-weight: 600;
            font-size: 0.9rem;
            letter-spacing: 0.5px;
            opacity: 0.9;
        }

        @media (max-width: 768px) {
            .nav-links { display:none; flex-direction:column; position:fixed; top:74px; left:0; width:100%; background:rgba(20,20,34,0.98); padding:2rem 0; gap:1.5rem; box-shadow: 0 10px 20px rgba(0,0,0,0.5); z-index: 1500; }
            body.light-mode .nav-links { background: #ffffff; }
            .nav-links.active { display:flex; }
            .hamburger { display:flex; }
            .document-container { padding: 1.5rem; }
            h1 { font-size: 2rem; }
        }

        /* =========================================================
           FOOTER
           ========================================================= */
        footer { background:#121629; color:#fafaff; text-align:center; padding:3rem 20px; margin-top: auto; }
        body.light-mode footer { background:#e9ecef; color:#22223b; }
        .sitemap a { color:#bdc3c7; margin:0 10px; text-decoration:none; transition: color 0.3s;}
        .sitemap a:hover { color: var(--accent); }
        .footer-orcid { display: inline-flex; align-items: center; justify-content: center; gap: 8px; margin-top: 1rem; color: #8e97a6; text-decoration: none;}
        .footer-orcid:hover { color: #fff; }
        body.light-mode .footer-orcid:hover { color: var(--accent); }
    </style>
</head>
<body>

    <!-- Fixed Header -->
    <header>
        <nav>
            <a href="index.html" class="logo">AURPHYX LLC</a>
            <ul class="nav-links" id="navLinks">
                <li><a href="index.html">Home</a></li>
                <li><a href="index.html#technologies">Technologies</a></li>
                <li><a href="explorer.html">Explorer</a></li>
                <li><a href="business-compliance.html">Compliance</a></li>
                <li class="dropdown">
                    <a class="dropbtn" style="cursor:pointer; color:#00F0FF;">Legal ▾</a>
                    <div class="dropdown-content">
                        <a href="privacy-policy.html">Privacy Policy</a>
                        <a href="terms-of-service.html" style="color:var(--accent);">Terms of Service</a>
                    </div>
                </li>
            </ul>
            <div class="header-controls">
                <button id="font-toggle" class="control-btn" title="Toggle Dyslexia-Friendly Font">A</button>
                <button id="theme-toggle" class="control-btn" title="Toggle Dark/Light Mode"><span id="theme-icon">🌙</span></button>
            </div>
            <div class="hamburger" id="hamburger"><span></span><span></span><span></span></div>
        </nav>
    </header>

    <!-- Main Content -->
    <main id="content">
        <div class="document-container">
            <div class="doc-header">
                <h1>Terms of Service</h1>
                <p class="last-updated">Last Updated: April 2026</p>
            </div>

            <div class="highlight">
                <p><strong>Welcome to the Forge.</strong> These Terms of Service ("Terms") govern your access to and use of the software, hardware, websites, mesh networking protocols, and AI platforms developed by Aurphyx LLC ("Aurphyx," "we," "us," or "our"). By initializing our software, participating in our Shard Mesh, or accessing our domains, you agree to be bound by these Terms.</p>
            </div>

            <h2>1. Nature of the Infrastructure</h2>
            <p>Aurphyx is not a traditional Software-as-a-Service (SaaS) provider. We build sovereign computing infrastructure, including the AuraOS microkernel, AuraFS decentralized storage, and g0dm0d3 orchestration frameworks. Because our architecture is designed for edge-computing and offline-first environments:</p>
            <ul>
                <li>You are the ultimate administrator of your local hardware and mesh nodes.</li>
                <li>Aurphyx does not actively monitor, moderate, or control the data transmitted locally across your instance of the Shard Mesh.</li>
            </ul>

            <h2>2. Cryptographic Responsibility</h2>
            <p>The Aurphyx ecosystem relies heavily on end-to-end encryption and cryptographic autonomy. <strong>Your Keys, Your Realm.</strong></p>
            <div class="important">
                <p class="legal-caps">Aurphyx LLC does not hold, store, or have access to your private cryptographic keys, recovery seeds, or biometric hashes used to secure your data within AuraFS or AuraOS. If you lose access to your authentication credentials, your data will be permanently irretrievable. Aurphyx LLC bears zero liability for data loss resulting from lost, stolen, or mismanaged keys.</p>
            </div>

            <h2>3. Acceptable Use and Network Integrity</h2>
            <p>While we advocate for digital sovereignty, your participation in the public-facing aspects of the Aurphyx network (such as global Meshwerk relays and public g0dm0d3 plugin repositories) requires adherence to the following code of conduct. You agree <strong>not</strong> to:</p>
            <ul>
                <li>Deploy malicious code, viruses, or self-replicating malware designed to disrupt the Shard Mesh or other users' local instances of AuraOS.</li>
                <li>Attempt to bypass, exploit, or reverse-engineer the licensing or authentication gateways of our proprietary enterprise software.</li>
                <li>Utilize Aurphyx infrastructure to facilitate illegal activity, including but not limited to the distribution of CSAM, terrorist financing, or human trafficking.</li>
            </ul>

            <h2>4. AI Interactions and Orchestration (Audry & g0dm0d3)</h2>
            <p>Aurphyx provides access to advanced AI frameworks, including the Audry multimodal consciousness and the g0dm0d3 terminal. You acknowledge that:</p>
            <ul>
                <li>AI models operate probabilistically. We do not guarantee the absolute accuracy, safety, or reliability of outputs generated by Audry or secondary models orchestrated via g0dm0d3.</li>
                <li>If you grant Audry or any AI agent "root" or administrative execution permissions over your local OS or file system, you assume full responsibility for any resulting automated actions, data modification, or system instability.</li>
            </ul>

            <h2>5. Intellectual Property</h2>
            <p>The Aurphyx ecosystem is a hybrid of open-source protocols and proprietary corporate infrastructure.</p>
            <ul>
                <li><strong>Open Source Components:</strong> Certain protocols, smart contracts, and SDKs released by Aurphyx are governed by their respective open-source licenses (e.g., MIT, GPL) as noted in their GitHub repositories.</li>
                <li><strong>Proprietary Assets:</strong> The Aurphyx brand, logos, AuraOS proprietary kernel extensions, Audry's core neural architecture, and specific hardware designs (Aura Gloves, AuraOrbs) remain the exclusive intellectual property of Aurphyx LLC. You may not distribute, clone, or monetize our proprietary assets without an explicit Enterprise License.</li>
            </ul>

            <h2>6. Disclaimer of Warranties</h2>
            <p>Unless covered by a specific physical hardware warranty agreement provided at the time of purchase (e.g., for Aura Tablets or Orbs), all software and digital infrastructure is provided strictly on an "as-is" basis.</p>
            <div class="important">
                <p class="legal-caps">To the maximum extent permitted by applicable law, Aurphyx LLC disclaims all warranties, express or implied, including but not limited to implied warranties of merchantability, fitness for a particular purpose, and non-infringement. We do not warrant that the software will be completely error-free, un-hackable, or continuously available.</p>
            </div>

            <h2>7. Limitation of Liability</h2>
            <p>In no event shall Aurphyx LLC, its founders, directors, employees, or affiliates be liable for any indirect, incidental, special, consequential, or punitive damages, including but not limited to loss of profits, data, goodwill, or business interruption, arising out of or in connection with your use or inability to use our sovereign infrastructure.</p>

            <h2>8. Governing Law and Jurisdiction</h2>
            <p>These Terms, and any disputes arising from them, shall be governed by and construed in accordance with the laws of the Commonwealth of Pennsylvania, United States, without regard to its conflict of law provisions. You agree to submit to the exclusive jurisdiction of the state and federal courts located in Erie County, Pennsylvania.</p>

            <h2>9. Modifications to the Terms</h2>
            <p>Aurphyx LLC reserves the right to modify or replace these Terms at any time to reflect updates to our technology ecosystem or regulatory changes. We will provide notice of significant material changes via our website or directly through AuraOS network broadcasts.</p>

            <h2>10. Contact Information</h2>
            <div class="contact-info">
                <h3>Legal & Compliance</h3>
                <p>For questions regarding these Terms of Service or to report a violation of the Acceptable Use policy, please contact our legal team:</p>
                
                <p>
                    <strong>Legal Department:</strong> legal@aurphyx.net<br>
                    <strong>General Information:</strong> info@aurphyx.net<br>
                </p>
                
                <p style="margin-top: 1rem;">
                    <strong>Headquarters:</strong><br>
                    Aurphyx LLC Legal Department<br>
                    502 W 7TH ST, STE 100<br>
                    ERIE, PA 16502-1333<br>
                    United States
                </p>
            </div>
        </div>
    </main>

    <!-- Footer -->
    <footer>
        <div class="sitemap">
            <a href="privacy-policy.html">Privacy Policy</a> • 
            <a href="terms-of-service.html">Terms of Service</a> • 
            <a href="business-compliance.html">Compliance</a> • 
            <a href="explorer.html">Explorer</a>
        </div>
        <a href="https://orcid.org/0009-0008-0539-1289" class="footer-orcid" target="_blank" rel="noopener noreferrer">
            <svg class="orcid-icon" viewBox="0 0 512 512" style="fill:currentColor; width:16px; height:16px;">
                <path d="M294.75 188.19h-45.92V342h47.47c67.62 0 83.12-51.34 83.12-76.91 0-41.64-26.54-76.9-84.67-76.9zM256 8C119 8 8 119 8 256s111 248 248 248 248-111 248-248S393 8 256 8zm-80.79 360.76h-29.84v-207.5h29.84v207.5zm-14.92-231.14a19.57 19.57 0 1 1 19.57-19.57 19.64 19.64 0 0 1-19.57 19.57zM300 369h-81V161.26h80.6c76.73 0 110.44 54.83 110.44 103.85C410 318.39 368.38 369 300 369z"/>
            </svg>
            ORCID: 0009-0008-0539-1289
        </a>
        <div style="margin-top:1rem; font-size:0.9rem; color: #8e97a6;">© 2026 Aurphyx LLC • All Rights Reserved • Erie, PA, United States</div>
    </footer>

    <script>
        /* =========================================================
           THEME TOGGLES & MOBILE MENU LOGIC
           ========================================================= */
        const themeToggle = document.getElementById('theme-toggle');
        const themeIcon = document.getElementById('theme-icon');
        const fontToggle = document.getElementById('font-toggle');
        const hamburger = document.getElementById('hamburger');
        const navLinks = document.getElementById('navLinks');

        function setTheme(isLight) {
            document.body.classList.toggle('light-mode', isLight);
            themeIcon.textContent = isLight ? '☀️' : '🌙';
            localStorage.setItem('theme', isLight ? 'light' : 'dark');
        }
        function setFont(isDyslexic) {
            document.body.classList.toggle('dyslexic-font', isDyslexic);
            localStorage.setItem('font', isDyslexic ? 'dyslexic' : 'standard');
        }

        // Check local storage so preferences carry over from index.html
        if (localStorage.getItem('theme') === 'light') setTheme(true);
        if (localStorage.getItem('font') === 'dyslexic') setFont(true);

        themeToggle.addEventListener('click', () => setTheme(!document.body.classList.contains('light-mode')));
        fontToggle.addEventListener('click', () => setFont(!document.body.classList.contains('dyslexic-font')));
        hamburger.addEventListener('click', () => navLinks.classList.toggle('active'));
    </script>
</body>
</html>