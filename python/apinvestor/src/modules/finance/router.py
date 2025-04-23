from fastapi import APIRouter, FastAPI

app = FastAPI()
router = APIRouter()

@router.get("/items/{item_id}")
async def read_item(item_id: int):
    return {"item_id": item_id}

app.include_router(router)


