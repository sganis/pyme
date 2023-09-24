from datetime import datetime, timedelta
from typing import Annotated
from fastapi import Depends, HTTPException, status, APIRouter
from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm
from sqlmodel import Session, select
from jose import jwt
from passlib.context import CryptContext
from api.db import get_session
from api.models import User, UserCreate, UserBase, Token


# to get a string like this run:
# openssl rand -hex 32
SECRET_KEY = "09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7"
ALGORITHM = "HS256"
TOKEN_EXPIRE_MINUTES = 60 * 24 * 7  # 1 week

hasher = CryptContext(schemes=["bcrypt"], deprecated="auto")
oauth2 = OAuth2PasswordBearer(tokenUrl="token")
router = APIRouter()



@router.post("/token", response_model=Token)
async def token(form: Annotated[OAuth2PasswordRequestForm, Depends()],
                session: Session = Depends(get_session)):
    user = session.exec(select(User)
                        .where(User.username==form.username)).first()
    if not user or not hasher.verify(form.password, user.password_hash):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )
    expire = datetime.utcnow() + timedelta(minutes=TOKEN_EXPIRE_MINUTES)
    data = { "sub": user.username, "exp": expire }
    token = jwt.encode(data, SECRET_KEY, algorithm=ALGORITHM)


    tok = Token(
        token=token, 
        type="bearer", 
        username=user.username
    )

    return tok



@router.post("/register", response_model=UserBase)
def f(user: UserCreate, session: Session = Depends(get_session)):
    if len(user.password) < 3:
        raise HTTPException(status_code=400, detail="Password polilicy error")
    u = session.exec(select(User).where(User.username==user.username)).first()
    if u:
        raise HTTPException(status_code=400, detail="User already exists")
    try:
        u = User(username=user.username, 
                 password_hash=hasher.hash(user.password))
        session.add(u)
        session.commit()
        session.refresh(u)
        return u
    except Exception as ex:
        print(f'error: {ex}')
        raise HTTPException(status_code=400, detail="Could not register user")

