// Import required modules
const express = require('express');
const path = require('path');

// Create an Express app
const app = express();

// Set the port
const PORT = process.env.PORT || 3000;

// Middleware to parse JSON bodies
app.use(express.json());

// Serve static files from the "public" folder
app.use(express.static(path.join(__dirname, 'public')));

// In-memory storage for admin sessions (in production, use Redis or database)
const adminSessions = new Set();

// Admin password (in production, use environment variables and hashed passwords)
const ADMIN_PASSWORD = 'emkprints2024';

// Route for admin login
app.post('/api/admin/login', (req, res) => {
    const { password } = req.body;
    
    if (!password) {
        return res.status(400).json({ 
            success: false, 
            message: 'Password is required' 
        });
    }
    
    if (password === ADMIN_PASSWORD) {
        // Generate a simple session token
        const sessionToken = Math.random().toString(36).substring(2, 15) + 
                           Math.random().toString(36).substring(2, 15);
        
        // Store session (expires after 1 hour)
        adminSessions.add(sessionToken);
        setTimeout(() => {
            adminSessions.delete(sessionToken);
        }, 3600000); // 1 hour
        
        return res.json({ 
            success: true, 
            message: 'Login successful',
            token: sessionToken 
        });
    } else {
        return res.status(401).json({ 
            success: false, 
            message: 'Incorrect password' 
        });
    }
});

// Middleware to verify admin session
function verifyAdminSession(req, res, next) {
    const token = req.headers.authorization?.replace('Bearer ', '');
    
    if (!token || !adminSessions.has(token)) {
        return res.status(401).json({ 
            success: false, 
            message: 'Unauthorized access' 
        });
    }
    
    next();
}

// Route to verify admin session
app.get('/api/admin/verify', verifyAdminSession, (req, res) => {
    res.json({ 
        success: true, 
        message: 'Session is valid' 
    });
});

// Route for admin logout
app.post('/api/admin/logout', verifyAdminSession, (req, res) => {
    const token = req.headers.authorization?.replace('Bearer ', '');
    adminSessions.delete(token);
    
    res.json({ 
        success: true, 
        message: 'Logged out successfully' 
    });
});

// Protected route example - for future admin operations
app.get('/api/admin/dashboard', verifyAdminSession, (req, res) => {
    res.json({ 
        success: true, 
        message: 'Welcome to admin dashboard',
        data: {
            // Add any admin-specific data here
            totalProducts: 10,
            activeOrders: 5
        }
    });
});

// Route for the homepage
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

// Error handling middleware
app.use((err, req, res, next) => {
    console.error(err.stack);
    res.status(500).json({ 
        success: false, 
        message: 'Something went wrong!' 
    });
});

// Handle 404 routes
app.use((req, res) => {
    res.status(404).json({ 
        success: false, 
        message: 'Route not found' 
    });
});

// Start the server
app.listen(PORT, () => {
    console.log(`Server running at http://localhost:${PORT}`);
});
