import { useNavigate } from "react-router";
import "../App.css";

function Home() {
  const navigate = useNavigate();
  return (
    <>
      <h1>Home Page</h1>
      <button onClick={() => navigate('/page')}>Test Page</button>
    </>
  );
}

export default Home;
