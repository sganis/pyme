import pytest 
from fastapi.testclient import TestClient
from sqlmodel import Session, SQLModel, create_engine
from sqlmodel.pool import StaticPool
from api.main import app
from api.db import get_session
from api.dependencies import authenticate
from api.routes.auth import hasher
from api.models import *

@pytest.fixture(name="session") 
def session_fixture(): 
    engine = create_engine(
        "sqlite://", connect_args={"check_same_thread": False}, poolclass=StaticPool
    )
    SQLModel.metadata.create_all(engine)
    with Session(engine) as session:
        yield session 


@pytest.fixture(name="client") 
def client_fixture(session: Session): 
    
    # memory db
    def get_session_override(): 
        return session
    
    # no authentication
    def authenticate_override(): 
        return session

    app.dependency_overrides[get_session] = get_session_override 
    app.dependency_overrides[authenticate] = authenticate_override 

    client = TestClient(app) 
    yield client 
    app.dependency_overrides.clear() 
    

def test_create_user(session: Session, client: TestClient): 
    role = Role(name="USER")
    session.add(role)
    session.commit()
    session.refresh(role)

    response = client.post(
        "/user/", json={
                "role_id": role.id,
                "username": "alice",
                "firstname": "alice",
                "lastname": "lastname",
                "lastname": "lastname",
                "email": "alice@gmail.com",
                "tel": "5555555",
                "city": "CABA",
                "country": "AR",
                "dob": "1975-01-01",
                "doctype": "DNI",
                "docnr": "2222222",
                "status": "OK",
                "remarks": "no remarks",
            }
        )
    data = response.json()
    assert response.status_code == 200
    assert data["firstname"] == "alice"
    assert data["city"] == "CABA"
    assert data["country"] == "AR"
    assert data["username"] == "alice"
    assert data["role"]["name"] == "USER"
    assert data.get("password_hash") == None

    obj = session.get(User, data["id"])
    assert obj
    assert obj.username == "alice"

def test_read_users(session: Session, client: TestClient):
    role = Role(name="USER")
    rec = User(username="alice", role=role)
    session.add(rec)
    session.commit()

    response = client.get("/user/")
    data = response.json()

    assert response.status_code == 200
    assert len(data) == 1
 
    assert data[0]["username"] == rec.username
    assert data[0]["id"] == rec.id
    assert data[0]["role"]["name"] == rec.role.name

    # do not get deleted    
    rec.deleted = True
    session.add(rec)
    session.commit()
    session.refresh(rec)
    response = client.get("/user/")
    data = response.json()
    print(data)
    assert response.status_code == 200
    assert len(data) == 0

# def test_read_user_username(session: Session, client: TestClient):
#     role = Role(name="USER")
#     rec = User(username="alice", role=role)
#     session.add(rec)
#     session.commit()

#     response = client.get(f"/user/{rec.username}")
#     data = response.json()

#     assert response.status_code == 200
#     assert data["username"] == rec.username
#     assert data["role"]["name"] == rec.role.name
    
def test_read_user(session: Session, client: TestClient):
    role = Role(name="USER")
    rec = User(username="alice", role=role)
    session.add(rec)
    session.commit()

    response = client.get(f"/user/{rec.id}")
    data = response.json()

    assert response.status_code == 200
    assert data["username"] == rec.username
    assert data["role"]["name"] == rec.role.name
    

def test_search(session: Session, client: TestClient):
    role = Role(name="USER")
    for i in range(5):
        rec = User(username=f"alice {i}", role=role)
        session.add(rec)
    session.commit()
    
    response = client.get(f"/user/?q=alice")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 5
    
    response = client.get(f"/user/?q=1")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 2

    response = client.get(f"/user/?q=alice&sortcol=username&desc=true")
    data = response.json()    
    assert response.status_code == 200
    assert data[0]["username"] == "alice 4"

    response = client.get(f"/user/?q=alice&limit=2")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 2

    response = client.get(f"/user/?q=alice&offset=4&limit=2")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 1
    assert data[0]["username"] == "alice 4"



def test_update_user(session: Session, client: TestClient):
    role = Role(name="USER")
    rec = User(username="alice", role=role)
    session.add(rec)
    session.commit()

    rec.disabled = True
    response = client.put(f"/user/", json={"username":"alice", "disabled": True})
    data = response.json()

    assert response.status_code == 200
    assert data["username"] == rec.username
    assert data["disabled"] == True
    assert data["role"]["name"] == rec.role.name

    obj = session.get(User, data["id"])
    assert obj
    assert obj.username == "alice"



def test_delete_user(session: Session, client: TestClient):
    rec = User(username="alice")
    session.add(rec)
    session.commit()

    assert session.get(User, rec.id)
    response = client.delete(f"/user/{rec.id}")
    assert response.status_code == 200
    assert session.get(User, rec.id).deleted

