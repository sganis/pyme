"""user

Revision ID: 079c83aa0016
Revises: 16336fdcf149
Create Date: 2023-09-16 13:00:16.004923

"""
from alembic import op
import sqlalchemy as sa
import sqlmodel


# revision identifiers, used by Alembic.
revision = '079c83aa0016'
down_revision = '16336fdcf149'
branch_labels = None
depends_on = None


def upgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.add_column('user', sa.Column('phone', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('company', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('cuit', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('docnumber', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('address', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('zip', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('province', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('ccnumber', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('ccexp', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('user', sa.Column('cccvv', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.drop_column('user', 'docnr')
    op.drop_column('user', 'tel')
    # ### end Alembic commands ###


def downgrade() -> None:
    # ### commands auto generated by Alembic - please adjust! ###
    op.add_column('user', sa.Column('tel', sa.VARCHAR(), autoincrement=False, nullable=True))
    op.add_column('user', sa.Column('docnr', sa.VARCHAR(), autoincrement=False, nullable=True))
    op.drop_column('user', 'cccvv')
    op.drop_column('user', 'ccexp')
    op.drop_column('user', 'ccnumber')
    op.drop_column('user', 'province')
    op.drop_column('user', 'zip')
    op.drop_column('user', 'address')
    op.drop_column('user', 'docnumber')
    op.drop_column('user', 'cuit')
    op.drop_column('user', 'company')
    op.drop_column('user', 'phone')
    # ### end Alembic commands ###
