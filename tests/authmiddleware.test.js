const httpMocks = require('node-mocks-http');
const jwt = require('jsonwebtoken');
const { protect } = require('../server/middleware/authmiddleware');

jest.mock('jsonwebtoken');

describe('authMiddleware', () => {
  let mockRequest;
  let mockResponse;
  let nextFunction;

  beforeEach(() => {
    mockRequest = httpMocks.createRequest();
    mockResponse = httpMocks.createResponse();
    nextFunction = jest.fn();
    process.env.JWT_SECRET = 'testsecret';
  });

  it('should verify a valid token and set req.user', () => {
    const userPayload = { id: 'user123', name: 'John Doe' };
    jwt.verify.mockReturnValue(userPayload);

    mockRequest.headers.authorization = 'Bearer validtoken';

    protect(mockRequest, mockResponse, nextFunction);

    expect(jwt.verify).toHaveBeenCalledWith('validtoken', process.env.JWT_SECRET);
    expect(mockRequest.user).toMatchObject(userPayload);
    expect(nextFunction).toHaveBeenCalled();
  });

  it('should return 401 if no token is provided', () => {
    protect(mockRequest, mockResponse, nextFunction);

    expect(mockResponse.statusCode).toBe(401);
    expect(JSON.parse(mockResponse._getData())).toEqual(expect.objectContaining({ message: 'Not authorized, no token' }));
  });

  it('should return 401 if token is invalid', () => {
    jwt.verify.mockImplementation(() => {
      throw new Error('Token is invalid');
    });

    mockRequest.headers.authorization = 'Bearer invalidtoken';

    protect(mockRequest, mockResponse, nextFunction);

    expect(mockResponse.statusCode).toBe(401);
    expect(JSON.parse(mockResponse._getData())).toEqual(expect.objectContaining({ message: 'Not authorized, token failed' }));
  });

  // Add more tests as needed for different scenarios
});
