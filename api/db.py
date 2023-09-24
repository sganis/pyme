import os
from sqlmodel import SQLModel, Session, create_engine
from dotenv import load_dotenv

load_dotenv() 

engine = create_engine(os.environ.get("DATABASE_URL"), 
                    #    echo=True,
                       pool_pre_ping=True,
                    #    connect_args={"check_same_thread": False}
                       )

def get_session():
    with Session(engine) as session:
        yield session

SQLModel.metadata.create_all(engine)
