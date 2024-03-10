const bcrypt = require('bcryptjs');

async function testBcrypt() {
  const password = 'hardcodedPassword';
  const hashedPassword = await bcrypt.hash(password, 10);
  const isMatch = await bcrypt.compare(password, hashedPassword);
  
  console.log('Hashed Password:', hashedPassword);
  console.log('Comparison Result:', isMatch); // This should be true if everything is working correctly
}

testBcrypt().catch(console.error);
