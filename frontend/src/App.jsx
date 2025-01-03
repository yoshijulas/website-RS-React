import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Login from "./pages/Login";
import Signup from "./pages/Signup";
import Profile from "./pages/Profile";
import Index from "./Index";

function App() {
	return (
		<Router>
			<div className="App">
				<Routes>
					<Route path="/" element={<Index />} />
					<Route path="/login" element={<Login />} />
					<Route path="/signup" element={<Signup />} />
					<Route path="/profile/:userId" element={<Profile />} />
				</Routes>
			</div>
		</Router>
	);
}

export default App;
