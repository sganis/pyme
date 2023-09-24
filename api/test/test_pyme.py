import pytest  # 
from datetime import datetime, timedelta, date
from fastapi.testclient import TestClient
from sqlmodel import Session, SQLModel, create_engine
from sqlmodel.pool import StaticPool
from api.main import app
from api.db import get_session
from api.dependencies import authenticate
from api.models import *

date = datetime(2020,1,1)
date_str = date.strftime('%Y-%m-%d')


@pytest.fixture(name="session") 
def session_fixture(): 
    engine = create_engine(
        "sqlite://", connect_args={"check_same_thread": False}, 
        poolclass=StaticPool
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
    

def test_create(session: Session, client: TestClient): 
    response = client.post(
        "/pyme/", json={
                    "date" : date_str,  
                    "customer" : 'customer',
                    "product" : 'A',
                    "quantity" : 1,
                    "price" : 50,
                    }
                )
    data = response.json()
    # print(data)
    assert response.status_code == 200
    assert data["date"] == date_str
    assert data["customer"] == 'customer'
    assert data["product"] == 'A'
    assert data["quantity"] == 1
    assert data["price"] == 50


    rec = session.get(Pyme, 1)
    assert rec.date == date_str
    assert rec.customer == 'customer'
    assert rec.product == 'A'
    assert rec.quantity == 1
    assert rec.price == 50



def test_read_all(session: Session, client: TestClient):  
    rec =  Pyme(date = date_str,
                customer = 'customer',
                product = 'A',
                quantity = 1,
                price = 50)
    session.add(rec)
    session.commit()
    session.refresh(rec)

    response = client.get("/pyme/")
    data = response.json()
    
    assert response.status_code == 200
    assert len(data) == 1
    assert data[0]["date"] == date_str
    assert data[0]["customer"] == 'customer'
    assert data[0]["product"] == 'A'
    assert data[0]["quantity"] == 1
    assert data[0]["price"] == 50
    
    # do not get deleted    
    rec.deleted = True
    session.add(rec)
    session.commit()
    session.refresh(rec)
    response = client.get("/pyme/")
    data = response.json()
    assert response.status_code == 200
    assert len(data) == 0

    

def test_read_one(session: Session, client: TestClient):
    # roomtype = RoomType(name="DOUBLE", num_guests=2)
    # room = Room(name="101", roomtype=roomtype)
    rec =  Pyme(date = date_str,
                customer = 'customer',
                product = 'A',
                quantity = 1,
                price = 50)
    session.add(rec)
    session.commit()
    session.refresh(rec)
    
    response = client.get(f"/pyme/{rec.id}")
    data = response.json()
    
    assert response.status_code == 200
    assert data["date"] == date_str
    assert data["customer"] == 'customer'
    assert data["product"] == 'A'
    assert data["quantity"] == 1
    assert data["price"] == 50
    

def test_search(session: Session, client: TestClient):
    for i in range(5):
        rec =  Pyme(date = date_str,
                customer = f'customer {i}',
                product = 'A',
                quantity = 1,
                price = 50)
        session.add(rec)
        session.commit()
    
    response = client.get(f"/pyme/?q=customer")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 5
    
    response = client.get(f"/pyme/?q=4")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 1

    response = client.get(f"/pyme/?q=customer&sortcol=customer&desc=true")
    data = response.json()    
    assert response.status_code == 200
    assert data[0]["customer"] == "customer 4"

    response = client.get(f"/pyme/?q=customer&limit=2")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 2

    response = client.get(f"/pyme/?q=customer&offset=4&limit=2")
    data = response.json()    
    assert response.status_code == 200
    assert len(data) == 1
    assert data[0]["customer"] == "customer 4"

  

def test_update(session: Session, client: TestClient):
    rec =  Pyme(date = date_str,
                customer = 'customer',
                product = 'A',
                quantity = 1,
                price = 50)
    session.add(rec)
    session.commit()
    session.refresh(rec)

    response = client.put(f"/pyme/", json={
            "id": rec.id,
            "customer": 'new customer',
        })
    data = response.json()

    assert response.status_code == 200
    assert data["customer"] == 'new customer'

    rec = session.get(Pyme, 1)
    assert rec.customer == 'new customer'

    

def test_delete(session: Session, client: TestClient):
    rec =  Pyme(date = date_str,
                customer = 'customer',
                product = 'A',
                quantity = 1,
                price = 50)
    session.add(rec)
    session.commit()
    session.refresh(rec)

    assert session.get(Pyme, rec.id)
    response = client.delete(f"/pyme/{rec.id}")
    assert response.status_code == 200
    assert session.get(Pyme, rec.id).deleted

