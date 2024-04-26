import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [furl, setfurl] = useState("https://rccdsb.ca/");
  const [menshow, setMenshow] = useState(false);
  const [noteshow, setNoteshow] = useState(false);
  const [urlshow, setUrlshow] = useState(false);
  // This will wait for the window to load, but you could
  // run this function on whatever trigger you want
 // 	invoke('close_splashscreen')
//})
 // invoke('close_splacescreen');
//  async function greet() {
  	//inputter.currentTarget.value
  	//() => setfurl("menu.html")
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    
   // setfurl(await invoke("greet", { name }));
 // }
  
  
  /*async function closesplash() {
    invoke('ose_splashscreen')
  }
  async function opensplash() {
    invoke('open_splashscreen')
  }
  */
  async function toggleurl() {
    setUrlshow((urlshow) => !urlshow)
  }
  async function togglemenu() {
    setMenshow((menshow) => !menshow)
    }
    //setBookshow((bookshow) => false)
 // async function setter(url) {
   // setfurl(await invoke("srchange", { url }));
    //setfurl((furl) => "The_Confession_Of_Saint_Augustine/index.html")
  //}
  //async function genermenu() {
    //setMen2
  //}
 //<embed type="text/html" href="/Books/City_Of_God_Volume_1.html" width="100%" height="10000"/>
//{() => setfurl("menu.html")}
//{menshow && <tr><td><button class="mensource" onClick={() => {setfurl("https://scholar.google.ca"); togglemenu(); setUrlshow(true)}}><h2>Google Scholar</h2></button></td></tr>}
  return (
    <div className="container">
    <header>
    	<button class="mentop" onClick={() => {togglemenu(); setUrlshow(false)}}>
    		<h2>Menu</h2>
    	</button>
    	</header>
    	<table class="menscroll">
    	{menshow && <tr><td><button class="mensource" onClick={() => {setfurl("https://rccdsb.ca/"); togglemenu(); setUrlshow(false)}}><h2>Bishop Dashboard</h2></button></td></tr>}
    	{menshow && <tr><td><h2>Research Sources</h2></td></tr>}
    	{menshow && <tr><td><button class="mensource" onClick={() => {setfurl("https://gutenberg.org/"); togglemenu(); setUrlshow(true)}}><h2>Project Gutenberg</h2></button></td></tr>}
    	{menshow && <tr><td><h2>Citation</h2></td></tr>}
    	{menshow && <tr><td><button class="mensource" onClick={() => {setfurl("https://www.citationmachine.net/apa"); togglemenu(); setUrlshow(true)}}><h2>APA</h2></button></td></tr>}
    	{menshow && <tr><td><button class="mensource" onClick={() => {setfurl("https://www.citationmachine.net/mla"); togglemenu(); setUrlshow(true)}}><h2>MLA</h2></button></td></tr>}
    	{menshow && <tr><td>Research Integrity<h2></h2></td></tr>}
    	</table>
    	{noteshow && <div id="notearea"><textfield></textfield></div>}
        {urlshow && <h4>{furl}</h4>}
      {!menshow && <iframe id="viewarea" src={furl} loading="lazy" />}
      </div>
  );
}
//onLoad={frameshow}
export default App;
