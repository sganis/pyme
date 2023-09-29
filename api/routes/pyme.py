from fastapi import Depends, HTTPException, APIRouter
from sqlmodel import Session, select, or_
from api.models import Pyme, User
from api.db import get_session
from api.dependencies import authenticate

router = APIRouter(
    dependencies=[Depends(authenticate)],
    responses={404: {"description": "Not found"}}
)

@router.get("/pyme/", response_model=list[Pyme])
def f(q: str=None, offset:int=0, limit:int=100, 
      sortcol:str='', desc:bool=False, 
      session: Session = Depends(get_session)):
    try:
        query = select(Pyme).where(Pyme.deleted == False)
        if q:
            query = query.where(Pyme.customer.ilike(f'%{q}%'))
        if sortcol:
            if not desc:
                query = query.order_by(getattr(Pyme, sortcol))
            else:
                query = query.order_by(getattr(Pyme, sortcol).desc())
        query = query.offset(offset).limit(limit)
        items = session.exec(query).all()     
        return items
    except Exception as ex:
        print(ex)
        raise HTTPException(status_code=400, detail=f"Error in Db")
    

    
@router.get("/pyme/{id}", response_model=Pyme)
def f(id: int, session: Session = Depends(get_session)):
    u = session.get(Pyme, id)
    if not u:
        raise HTTPException(status_code=404, detail="Record not found")
    return u


@router.post("/pyme/", response_model=Pyme)
def f(obj: Pyme, session: Session = Depends(get_session)):
    try:        
        b = Pyme(date = obj.date,
                customer = obj.customer.upper(),
                product = obj.product,
                quantity = obj.quantity,
                price = obj.price,
        )
        session.add(b)
        session.commit()
        session.refresh(b)
        return b
    except Exception as ex:
        print(f'error: {ex}')
        raise HTTPException(status_code=400, detail="Could not add record")


@router.put("/pyme/", response_model=Pyme)
def f(obj: Pyme, session: Session = Depends(get_session)):
    obj_db = session.get(Pyme, obj.id)
    if not obj_db:
        raise HTTPException(status_code=404, detail="Record not found")

    for key, value in obj.dict(exclude_unset=True).items():
        setattr(obj_db, key, value)

    session.add(obj_db)
    session.commit()
    session.refresh(obj_db)
    return obj_db


@router.delete("/pyme/{id}", response_model=Pyme)
def f(id: int, session: Session = Depends(get_session)):
    obj_db = session.get(Pyme, id)
    if not obj_db:
        raise HTTPException(status_code=404, detail="Record not found")

    obj_db.deleted = True
    session.add(obj_db)
    session.commit()
    session.refresh(obj_db)
    return obj_db


@router.get("/pyme/customer/", response_model=list[str])
def f(session: Session = Depends(get_session)):
    try:
        query = select(Pyme.customer.distinct()).where(Pyme.deleted == False)
        items = session.exec(query).all()     
        return items
    except Exception as ex:
        print(ex)
        raise HTTPException(status_code=400, detail=f"Error in Db")
