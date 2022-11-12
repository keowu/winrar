<h1 align="center">Simple Winrar Crack in Rust</h1>

<img src="https://github.com/keowu/testesreadme/blob/main/pic/rustacean_oh.png?raw=true" width="6912" height="400">

<h2>What does it do ?</h2>

<p>A simple project that allows you to modify the license check used by WinRaR, "RegKey" from virtual memory using the Rust language, made to play with reverse engineering using the Rustacean power.</p>

<h2>How it works ?</h2>

<p>From the memory of the process itself and the "binding" of WinApi an injected DLL modifies one instruction by another ignoring the check made by WinRaR (For that execution), this is not a Crack as the memory is free, no modified binary is shared .</p>

<h2>Screenshots</h2>

<p>Before modification:</p>

<img src="https://github.com/keowu/testesreadme/blob/main/pic/2.PNG?raw=true">

<p>After modified:</p>

<img src="https://github.com/keowu/testesreadme/blob/main/pic/3.PNG?raw=true">

<h2>How to use ?</h2>


<p>It's simple, load the compiled lib into process memory and be happy.</p>


<h2>How to build ?</h2>

<p>Crustaceans, joking aside, follow the steps:</p>

<ul>
  <li>cargo build --lib --release</li>
  <li>Get compiled lib from target folder</li>
</ul>

<h3>Information</h3>

<p>This project is for Reverse Engineering studies, I hope it will help you, and if you can afford it, buy an original license for any type of software.</p>
