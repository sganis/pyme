import pytest 
from fastapi.testclient import TestClient
from sqlmodel import Session, SQLModel, create_engine
from sqlmodel.pool import StaticPool
from api.main import app
from api.db import get_session
from api.routes.auth import hasher, SECRET_KEY, ALGORITHM
from api.models import User
from jose import jwt


@pytest.fixture(name="session")  # 
def session_fixture():  # 
    engine = create_engine(
        "sqlite://", 
        connect_args={"check_same_thread": False}, 
        poolclass=StaticPool
    )
    SQLModel.metadata.create_all(engine)
    with Session(engine) as session:
        yield session  # 


@pytest.fixture(name="client")  # 
def client_fixture(session: Session):  # 
    def get_session_override():  # 
        return session

    app.dependency_overrides[get_session] = get_session_override  # 

    client = TestClient(app)  # 
    yield client  # 
    app.dependency_overrides.clear()  # 
    


def test_register(session: Session, client: TestClient):  # 
    response = client.post(
        "/register", json={
                "username": "alice",
                "password": "secret",
            }
        )
    data = response.json()
    assert response.status_code == 200
    assert data["username"] == "alice"
    assert data.get("password_hash") == None
    u1 = session.get(User, 1)
    assert hasher.verify('secret', u1.password_hash)




def test_auth(session: Session, client: TestClient):
    u1 = User(username="alice", password_hash=hasher.hash('secret'))
    session.add(u1)
    session.commit()
    session.refresh(u1)

    response = client.post(
        "/token", data={"username": "alice", "password": "secret"}
    )
    data = response.json()
    assert response.status_code == 200
    assert data["type"] == 'bearer'
    token = data["token"]
    payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
    username = payload.get("sub") 
    assert username == "alice"
    u1 = session.get(User, 1)
    assert hasher.verify('secret', u1.password_hash)

    # correct token
    response = client.get(
        "/profile", headers={"Authorization": f"Bearer {token}"}
    )
    data = response.json()
    assert response.status_code == 200
    assert data["username"] == 'alice'

    # bad token
    response = client.get(
        "/profile", headers={"Authorization": f"Bearer bad"}
    )
    data = response.json()
    assert response.status_code == 401
    assert data["detail"] == 'Could not validate credentials'



def test_auth_wrong_password(session: Session, client: TestClient):
    u1 = User(username="alice", password_hash=hasher.hash('secret'))
    session.add(u1)
    session.commit()
    session.refresh(u1)

    response = client.post(
        "/token", data={"username": "alice", "password": "1234"}
    )
    data = response.json()
    assert response.status_code == 401
    assert data["detail"] == 'Incorrect username or password'
    

    
