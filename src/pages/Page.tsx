import { useNavigate } from "react-router-dom";
import "../App.css";
import { invoke } from "@tauri-apps/api/core";

function Page() {
  const navigate = useNavigate();
  return (
    <>
      <h1>Second Page</h1>
      <button onClick={() => navigate('/')}>Home</button>
      
      <button onClick={async () => {await invoke('upload_user')}}>Upload</button>
    </>
  );
}

export default Page;
