export default function (sequelize, DataTypes) {
	return sequelize.define(
		'User',
		{
			id: { type: DataTypes.BIGINT, primaryKey: true },
			name: { type: DataTypes.TEXT, allowNull: false },
			avatar: { type: DataTypes.TEXT, allowNull: true }
		},
		{
			sequelize,
			tableName: 'user',
			timestamps: false
		}
	);
}
