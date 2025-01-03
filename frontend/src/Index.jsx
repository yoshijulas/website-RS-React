import React from "react";
import { Link } from "react-router-dom";

const Index = () => {
	const isLoggedIn = false; // Change this based on your authentication logic

	return (
		<div className="min-h-screen flex items-center justify-center bg-gray-100">
			<div className="text-center">
				<h1 className="text-4xl font-bold mb-6">Welcome to the Website</h1>

				{/* Conditionally render login/signup or profile links */}
				{!isLoggedIn ? (
					<div>
						<Link to="/login" className="text-blue-500 hover:underline">
							Login
						</Link>{" "}
						|
						<Link to="/signup" className="text-blue-500 hover:underline">
							Signup
						</Link>
					</div>
				) : (
					<div>
						<Link to="/profile" className="text-blue-500 hover:underline">
							Profile
						</Link>
					</div>
				)}
			</div>
		</div>
	);
};

export default Index;
