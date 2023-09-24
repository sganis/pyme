from pydantic import condecimal
from sqlmodel import (SQLModel, Field,  Relationship, 
                      text, Column, TIMESTAMP, UniqueConstraint, Index)
from datetime import datetime
from typing import Optional



class Token(SQLModel):
    token: str
    type: str
    username: str



class Table(SQLModel):
    id         : Optional[int] = Field(default=None, primary_key=True, index=True)   
    deleted     : bool = Field(default=False, index=True)   
    created    : Optional[datetime] = Field(sa_column=Column(
        TIMESTAMP(timezone=True),
        nullable=False,
        server_default=text("CURRENT_TIMESTAMP"),
    ))
    updated    : Optional[datetime] = Field(sa_column=Column(
        TIMESTAMP(timezone=True),
        nullable=False,
        server_default=text("CURRENT_TIMESTAMP"),
        server_onupdate=text("CURRENT_TIMESTAMP"),
    ))
    



##########################################################################
# users
##########################################################################
class UserBase(Table):
    username    : Optional[str] = Field(index=True)
    
class User(UserBase, table=True):
    password_hash   : Optional[str] = None
 
class UserCreate(UserBase): 
    password        : str


##########################################################################
# pyme 
##########################################################################
class Pyme(Table, table=True):
    date        : Optional[str] = Field(index=True)
    customer    : Optional[str] = Field(index=True)
    product     : Optional[str] = Field(index=True)
    quantity    : Optional[int]
    price       : Optional[int]



